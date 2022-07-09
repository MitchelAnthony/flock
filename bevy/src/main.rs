use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;

// use flock_gui_plugin::FlockGuiPlugin;
use flock_player_plugin::FlockPlayerPlugin;
// use flock_state_plugin::FlockStatePlugin;
// use flock_tilemap_plugin::FlockTilemapPlugin;

// use crate::particle_effect::{
//     emit_particles, spawn_particle_spawner, update_particle_lifetime, ParticleColor, ParticleSize,
//     ParticleVelocity,
// };

// mod particle_effect;

fn main() {
    let mut app = App::new();

    app
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "My Awesome Game!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        // .add_plugin(FlockGuiPlugin)
        .add_plugin(FlockPlayerPlugin)
        // .add_plugin(FlockStatePlugin)
        // .add_plugin(FlockTilemapPlugin)
        .add_system(exit_on_esc_system)
        // .add_system(b.after(a))
        // .add_startup_system(spawn_particle_spawner)
        // .add_system(emit_particles)
        // .add_system(update_particle_lifetime)
        // .add_system_set(SystemSet::on_enter(AppState::Main).with_system(transition_settings));
    ;

    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());

        // app.register_inspectable::<ParticleSize>()
        //     .register_inspectable::<ParticleColor>()
        //     .register_inspectable::<ParticleVelocity>();
    }

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// fn transition_settings(mut app_state: ResMut<State<AppState>>) {
//     println!("Switching to Settings state.");
//
//     if app_state.set(AppState::Settings).is_err() {
//         println!("Could not switch state. State is {:?}", app_state.current());
//     }
// }
