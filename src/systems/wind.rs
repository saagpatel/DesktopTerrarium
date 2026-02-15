use crate::components::WindLeaf;
use crate::resources::{WeatherState, WeatherType};
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct WindAssets {
    pub leaf_handle: Handle<Image>,
    pub spawn_timer: Timer,
}

pub fn setup_wind_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(WindAssets {
        leaf_handle: asset_server.load("particles/leaf.png"),
        spawn_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
    });
}

pub fn wind_spawn_system(
    mut commands: Commands,
    mut wind_assets: ResMut<WindAssets>,
    weather: Res<WeatherState>,
    time: Res<Time>,
) {
    // Only spawn during Wind weather
    if weather.current != WeatherType::Wind && weather.target != WeatherType::Wind {
        return;
    }

    wind_assets.spawn_timer.tick(time.delta());

    if wind_assets.spawn_timer.just_finished() {
        let mut rng = rand::thread_rng();

        // Random spawn chance (3-8 per second)
        if rng.gen::<f32>() < 0.5 {
            commands.spawn((
                Sprite {
                    image: wind_assets.leaf_handle.clone(),
                    ..default()
                },
                Transform::from_xyz(
                    450.0, // Start from right edge
                    rng.gen_range(-200.0..200.0),
                    50.0,
                ),
                WindLeaf {
                    velocity: Vec2::new(rng.gen_range(-200.0..-100.0), rng.gen_range(-20.0..20.0)),
                    rotation_speed: rng.gen_range(-3.0..3.0),
                    lifetime: 8.0,
                },
            ));
        }
    }
}

pub fn wind_update_system(
    mut commands: Commands,
    mut leaves: Query<(Entity, &mut WindLeaf, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut leaf, mut transform) in &mut leaves {
        // Move the leaf
        transform.translation.x += leaf.velocity.x * time.delta_secs();
        transform.translation.y += leaf.velocity.y * time.delta_secs();

        // Rotate
        transform.rotate_z(leaf.rotation_speed * time.delta_secs());

        // Update lifetime
        leaf.lifetime -= time.delta_secs();

        // Despawn if expired or off-screen left
        if leaf.lifetime <= 0.0 || transform.translation.x < -450.0 {
            commands.entity(entity).despawn();
        }
    }
}
