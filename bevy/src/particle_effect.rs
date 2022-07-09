use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Clone, Copy, Inspectable)]
pub struct ParticleSize {
    start: f32,
    end: f32,
}

#[derive(Component, Clone, Copy, Inspectable)]
pub struct ParticleColor {
    start: Color,
    end: Color,
}

#[derive(Component, Clone, Copy, Inspectable)]
pub struct ParticleVelocity {
    start: Vec2,
    end: Vec2,
}

#[derive(Component)]
pub struct ParticleSpawner {
    rate: f32,
    amount_per_burst: usize,
    position_variance: f32,
    particle_lifetime: f32,
    particle_size: Option<ParticleSize>,
    particle_velocity: Option<ParticleVelocity>,
    particle_color: Option<ParticleColor>,
    timer: Timer,
    particles: usize,
}

#[derive(Component)]
pub struct Particle {
    lifetime: Timer,
}

pub fn emit_particles(
    mut commands: Commands,
    mut spawners: Query<(Entity, &mut ParticleSpawner)>,
    time: Res<Time>,
) {
    for (entity, mut spawner) in spawners.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.just_finished() && spawner.particles < 25 {
            for _i in 0..spawner.amount_per_burst {
                let particle = commands
                    .spawn()
                    .insert(Particle {
                        lifetime: Timer::from_seconds(spawner.particle_lifetime, false),
                    })
                    .insert(Name::new("Particle"))
                    .id();

                let mut sprite = SpriteBundle::default();
                sprite.transform.translation = Vec3::new(
                    spawner.position_variance * (100.0 * rand::random::<f32>() - 1.0),
                    spawner.position_variance * (100.0 * rand::random::<f32>() - 1.0),
                    0.0,
                );

                if let Some(size) = spawner.particle_size {
                    // sprite.sprite.custom_size = Some(Vec2::splat(size.start));
                    sprite.sprite.custom_size = Some(Vec2::splat(10.));
                    commands.entity(particle).insert(size);
                }
                if let Some(color) = spawner.particle_color {
                    sprite.sprite.color = color.start;
                    commands.entity(particle).insert(color);
                }
                if let Some(velocity) = spawner.particle_velocity {
                    commands.entity(particle).insert(velocity);
                }

                commands.entity(particle).insert_bundle(sprite);
                spawner.particles += 1;
            }

            break;
        }
    }
}

// Startup system
pub fn spawn_particle_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(ParticleSpawner {
            rate: 0.01,
            timer: Timer::from_seconds(0.05, true),
            amount_per_burst: 3,
            position_variance: 2.00,
            particle_lifetime: 202.5,
            particle_size: Some(ParticleSize {
                start: 0.1,
                end: 0.0,
            }),
            particle_velocity: Some(ParticleVelocity {
                start: Vec2::new(0.4, 0.5),
                end: Vec2::new(0.8, 0.2),
            }),
            particle_color: Some(ParticleColor {
                start: Color::WHITE,
                end: Color::ORANGE,
            }),
            particles: 0,
        });
}

pub fn update_particle_lifetime(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Particle)>,
    time: Res<Time>,
) {
    for (entity, mut particle) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
