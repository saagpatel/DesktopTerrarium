use crate::components::RainDrop;
use crate::resources::FeatureToggles;
use crate::resources::WeatherState;
use crate::resources::WeatherType;
use crate::systems::setup::SceneAssetHandles;
use bevy::prelude::*;
use rand::Rng;

const RAIN_SPAWN_RATE: f32 = 15.0; // drops per second

#[derive(Resource)]
pub struct RainAssets {
    pub spawn_timer: Timer,
}

pub fn setup_rain_assets(mut commands: Commands) {
    commands.insert_resource(RainAssets {
        spawn_timer: Timer::from_seconds(1.0 / RAIN_SPAWN_RATE, TimerMode::Repeating),
    });
}

pub fn rain_spawn_system(
    mut commands: Commands,
    mut rain_assets: ResMut<RainAssets>,
    scene_assets: Res<SceneAssetHandles>,
    toggles: Res<FeatureToggles>,
    weather: Res<WeatherState>,
    time: Res<Time>,
) {
    if !toggles.weather_particles_enabled() {
        return;
    }

    // Only spawn during Rain weather
    if weather.current != WeatherType::Rain && weather.target != WeatherType::Rain {
        return;
    }

    // Adjust spawn rate based on transition
    let spawn_multiplier = if weather.current == WeatherType::Rain {
        if weather.target == WeatherType::Rain {
            1.0 // Fully raining
        } else {
            1.0 - weather.transition_progress // Fading out
        }
    } else {
        weather.transition_progress // Fading in
    };

    if spawn_multiplier <= 0.0 {
        return;
    }

    rain_assets.spawn_timer.tick(time.delta());

    if rain_assets.spawn_timer.just_finished() {
        let mut rng = rand::thread_rng();

        // Spawn rain drops based on multiplier
        if rng.gen::<f32>() < spawn_multiplier {
            commands.spawn((
                Mesh3d(scene_assets.cuboid_mesh.clone()),
                MeshMaterial3d(scene_assets.rain_material.clone()),
                Transform::from_xyz(
                    rng.gen_range(-2.6..2.6),
                    rng.gen_range(3.8..4.8),
                    rng.gen_range(-1.9..1.9),
                )
                .with_scale(Vec3::new(0.02, 0.26, 0.02)),
                RainDrop {
                    velocity: Vec3::new(
                        rng.gen_range(-0.05..0.05),
                        rng.gen_range(-4.6..-3.4),
                        rng.gen_range(-0.12..0.12),
                    ),
                    lifetime: 1.8,
                },
            ));
        }
    }
}

pub fn rain_update_system(
    mut commands: Commands,
    toggles: Res<FeatureToggles>,
    mut rain: Query<(Entity, &mut RainDrop, &mut Transform)>,
    time: Res<Time>,
) {
    if !toggles.weather_particles_enabled() {
        for (entity, _, _) in &mut rain {
            commands.entity(entity).despawn();
        }
        return;
    }

    for (entity, mut drop, mut transform) in &mut rain {
        // Move the raindrop
        transform.translation += drop.velocity * time.delta_secs();
        transform.rotate_local_z(2.4 * time.delta_secs());

        // Update lifetime
        drop.lifetime -= time.delta_secs();

        // Despawn if expired or below the terrarium volume
        if drop.lifetime <= 0.0 || transform.translation.y < -0.95 {
            commands.entity(entity).despawn();
        }
    }
}

// Entity count safety check
pub fn rain_limit_system(rain: Query<Entity, With<RainDrop>>) {
    if rain.iter().count() > 200 {
        warn!("Rain particle count exceeded 200, consider reducing spawn rate");
    }
}
