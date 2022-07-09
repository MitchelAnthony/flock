use benimator::{AnimationPlugin, *};
use bevy::asset::LoadState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::*;
use std::time::Duration;

pub struct FlockPlayerPlugin;

impl Plugin for FlockPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayerLoadingState::Loading)
            .add_plugin(AnimationPlugin::default())
            .add_startup_system(setup)
            .add_system(handle_input)
            .add_system_set(
                SystemSet::on_enter(PlayerLoadingState::Loading).with_system(load_textures),
            )
            .add_system_set(
                SystemSet::on_update(PlayerLoadingState::Loading).with_system(check_textures),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerLoadingState::Finished).with_system(build_texture_atlas),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerLoadingState {
    Loading,
    Finished,
}

#[derive(Copy, Clone)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

const SPEED: f32 = 5.;

pub struct IdleAnimationTextures {
    asset_handles: Vec<HandleUntyped>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player/idle/chrFemaleExplorer1Idle1.png"),
            transform: Transform {
                translation: Vec3::new(50., 50., 100.),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Name::new("PlayerEntity"));
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles = asset_server
        .load_folder("player/idle")
        .expect("Assets should be available");

    commands.insert_resource(IdleAnimationTextures {
        asset_handles: handles,
    });
}

fn check_textures(
    mut state: ResMut<State<PlayerLoadingState>>,
    resource: ResMut<IdleAnimationTextures>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(resource.asset_handles.iter().map(|handle| handle.id))
    {
        state.set(PlayerLoadingState::Finished).unwrap();
    }
}

fn build_texture_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    resource: ResMut<IdleAnimationTextures>,
    mut images: ResMut<Assets<Image>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in resource.asset_handles.iter() {
        let image = images.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), image);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut images).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    // commands.spawn_bundle(SpriteSheetBundle {
    //     sprite: TextureAtlasSprite::new(0),
    //     texture_atlas: atlas_handle,
    //     ..default()
    // });
    let animation_handle = animations.add(SpriteSheetAnimation::from_range(
        0..=5,
        Duration::from_millis(200),
    ));

    commands
        // Spawn a bevy sprite-sheet
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        // Insert the asset handle of the animation
        .insert(animation_handle)
        // Start the animation immediately. Remove this component in order to pause the animation.
        .insert(Play);
}

fn handle_input(
    mut input_event_reader: EventReader<KeyboardInput>,
    mut query: Query<(&mut Transform, &mut Sprite), With<Player>>,
) {
    let mut input_action: Option<Action> = None;
    for event in input_event_reader.iter() {
        // Map input to action
        if event.state == ElementState::Pressed {
            if let Some(key) = event.key_code {
                match key {
                    KeyCode::Up => {
                        input_action = Some(Action::Up);
                    }
                    KeyCode::Down => {
                        input_action = Some(Action::Down);
                    }
                    KeyCode::Left => {
                        input_action = Some(Action::Left);
                    }
                    KeyCode::Right => {
                        input_action = Some(Action::Right);
                    }
                    _ => {}
                }
            }
        }

        if let Some(action) = input_action {
            let (mut transform, mut sprite) = query.single_mut(); // Unique for now

            match action {
                Action::Up => {
                    transform.translation.y += SPEED;
                }
                Action::Down => {
                    transform.translation.y -= SPEED;
                }
                Action::Left => {
                    transform.translation.x -= SPEED;
                    sprite.flip_x = true;
                }
                Action::Right => {
                    transform.translation.x += SPEED;
                    sprite.flip_x = false;
                }
            }
        }
    }
}
