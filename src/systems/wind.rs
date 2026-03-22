use crate::components::WindLeaf;
use crate::resources::{FeatureToggles, WeatherState, WeatherType};
use crate::systems::setup::SceneAssetHandles;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct WindAssets {
    pub spawn_timer: Timer,
}

pub fn setup_wind_assets(mut commands: Commands) {
    commands.insert_resource(WindAssets {
        spawn_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
    });
}

pub fn wind_spawn_system(
    mut commands: Commands,
    mut wind_assets: ResMut<WindAssets>,
    scene_assets: Res<SceneAssetHandles>,
    toggles: Res<FeatureToggles>,
    weather: Res<WeatherState>,
    time: Res<Time>,
) {
    if !toggles.weather_particles_enabled() {
        return;
    }

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
                Mesh3d(scene_assets.cuboid_mesh.clone()),
                MeshMaterial3d(scene_assets.wind_leaf_material.clone()),
                Transform::from_xyz(3.4, rng.gen_range(0.2..2.8), rng.gen_range(-1.8..2.1))
                    .with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        rng.gen_range(-0.6..0.6),
                        rng.gen_range(-0.4..0.4),
                        rng.gen_range(-1.2..1.2),
                    ))
                    .with_scale(Vec3::new(0.12, 0.02, 0.18)),
                WindLeaf {
                    velocity: Vec3::new(
                        rng.gen_range(-2.8..-1.4),
                        rng.gen_range(-0.15..0.22),
                        rng.gen_range(-0.35..0.35),
                    ),
                    rotation_speed: rng.gen_range(-3.0..3.0),
                    lifetime: 4.5,
                },
            ));
        }
    }
}

pub fn wind_update_system(
    mut commands: Commands,
    toggles: Res<FeatureToggles>,
    mut leaves: Query<(Entity, &mut WindLeaf, &mut Transform)>,
    time: Res<Time>,
) {
    if !toggles.weather_particles_enabled() {
        for (entity, _, _) in &mut leaves {
            commands.entity(entity).despawn();
        }
        return;
    }

    for (entity, mut leaf, mut transform) in &mut leaves {
        // Move the leaf
        transform.translation += leaf.velocity * time.delta_secs();
        transform.translation.y +=
            (time.elapsed_secs() * 1.8 + transform.translation.z).sin() * 0.08 * time.delta_secs();

        // Rotate
        transform.rotate_z(leaf.rotation_speed * time.delta_secs());
        transform.rotate_x(leaf.rotation_speed * 0.4 * time.delta_secs());

        // Update lifetime
        leaf.lifetime -= time.delta_secs();

        // Despawn if expired or off-screen left
        if leaf.lifetime <= 0.0 || transform.translation.x < -3.4 {
            commands.entity(entity).despawn();
        }
    }
}
