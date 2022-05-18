use bevy::prelude::*;
use flock_state_plugin::AppState;

pub struct FlockGuiPlugin;

impl Plugin for FlockGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::Tick>()
            .insert_resource(resources::Test(0))
            .add_startup_system(systems::setup)
            .add_system(systems::show_count)
            .add_system(systems::handle_tick)
            .add_system(systems::button_interaction);
    }
}

mod systems {
    use super::*;
    use crate::resources::{ButtonSound, Test};

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn_bundle(UiCameraBundle::default());
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Counter: 0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        });

        // Button
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..default()
                });
            });

        let sound = asset_server.load("sounds/breakout_collision.ogg");
        commands.insert_resource(ButtonSound(sound));
    }

    #[allow(clippy::type_complexity)]
    pub fn button_interaction(
        mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
        mut tick_writer: EventWriter<events::Tick>,
        mut windows: ResMut<Windows>,
        audio: Res<Audio>,
        button_sound: Res<ButtonSound>,
    ) {
        let window = windows.primary_mut();
        for (interaction, mut color) in query.iter_mut() {
            match interaction {
                Interaction::Clicked => {
                    *color = Color::rgb(0.0, 0.5, 0.0).into();
                    window.set_cursor_icon(CursorIcon::Wait);
                    tick_writer.send(events::Tick);
                    audio.play(button_sound.0.clone());
                }
                Interaction::Hovered => {
                    *color = Color::rgb(0.25, 0.25, 0.25).into();
                    window.set_cursor_icon(CursorIcon::Hand);
                }
                Interaction::None => {
                    *color = Color::rgb(0.15, 0.15, 0.15).into();
                    window.set_cursor_icon(CursorIcon::Default);
                }
            }
        }
    }

    pub fn show_count(test_resource: Res<Test>, mut query: Query<&mut Text>) {
        for mut text in query.iter_mut() {
            if !text.sections[0].value.contains("Counter") {
                continue;
            }

            text.sections[0].value = format!("Counter: {}", test_resource.0);
        }
    }

    pub fn handle_tick(
        mut tick_reader: EventReader<events::Tick>,
        mut test_resource: ResMut<Test>,
        mut app_state: ResMut<State<AppState>>,
    ) {
        for _ in tick_reader.iter() {
            test_resource.0 += 1;

            if test_resource.0 > 0 && test_resource.0 % 10 == 0 {
                println!("Switching to Main state.");

                if app_state.set(AppState::Main).is_err() {
                    println!("Could not switch state. State is {:?}", app_state.current());
                }
            }
        }
    }
}

mod resources {
    use super::*;

    #[derive(Debug)]
    pub struct Test(pub usize);

    pub struct ButtonSound(pub Handle<AudioSource>);
}

mod components {
    use super::*;

    #[derive(Component)]
    pub struct Label(String);
}

mod events {
    pub struct Tick;
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
