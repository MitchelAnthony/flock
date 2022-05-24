use bevy::{input::system::exit_on_esc_system, prelude::*};
use flock_gui_plugin::FlockGuiPlugin;
use flock_state_plugin::{AppState, FlockStatePlugin};
use flock_tilemap_plugin::FlockTilemapPlugin;

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Test!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(FlockGuiPlugin)
        .add_plugin(FlockStatePlugin)
        .add_plugin(FlockTilemapPlugin)
        .add_system(exit_on_esc_system)
        .add_system_set(SystemSet::on_enter(AppState::Main).with_system(transition_settings))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn transition_settings(mut app_state: ResMut<State<AppState>>) {
    println!("Switching to Settings state.");

    if app_state.set(AppState::Settings).is_err() {
        println!("Could not switch state. State is {:?}", app_state.current());
    }
}
