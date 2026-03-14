use crate::components::FogWisp;
use crate::resources::{FeatureToggles, WeatherState, WeatherType};
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct FogAssets {
    pub fog_wisp_handle: Handle<Image>,
}

pub fn setup_fog_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FogAssets {
        fog_wisp_handle: asset_server.load("particles/fog_wisp.png"),
    });
}

pub fn fog_spawn_system(
    mut commands: Commands,
    fog_assets: Res<FogAssets>,
    toggles: Res<FeatureToggles>,
    weather: Res<WeatherState>,
    existing_fog: Query<&FogWisp>,
) {
    if !toggles.weather_particles_enabled() {
        return;
    }

    // Only manage fog during Fog weather
    if weather.current != WeatherType::Fog && weather.target != WeatherType::Fog {
        return;
    }

    let current_count = existing_fog.iter().count();
    let target_count = if weather.current == WeatherType::Fog {
        8
    } else {
        0
    };

    // Spawn wisps if we need more
    if current_count < target_count {
        let mut rng = rand::thread_rng();
        commands.spawn((
            Sprite {
                image: fog_assets.fog_wisp_handle.clone(),
                ..default()
            },
            Transform::from_xyz(
                rng.gen_range(-450.0..450.0),
                rng.gen_range(-250.0..250.0),
                50.0,
            ),
            FogWisp {
                drift_speed: rng.gen_range(10.0..30.0),
                alpha_phase: rng.gen_range(0.0..std::f32::consts::TAU),
            },
        ));
    }
}

pub fn fog_update_system(
    mut commands: Commands,
    toggles: Res<FeatureToggles>,
    mut fog: Query<(Entity, &mut FogWisp, &mut Transform, &mut Sprite)>,
    weather: Res<WeatherState>,
    time: Res<Time>,
) {
    if !toggles.weather_particles_enabled() {
        for (entity, _, _, _) in &mut fog {
            commands.entity(entity).despawn();
        }
        return;
    }

    let should_have_fog = weather.current == WeatherType::Fog || weather.target == WeatherType::Fog;

    for (entity, mut wisp, mut transform, mut sprite) in &mut fog {
        if !should_have_fog {
            commands.entity(entity).despawn();
            continue;
        }

        // Drift horizontally
        transform.translation.x += wisp.drift_speed * time.delta_secs();

        // Wrap around screen
        if transform.translation.x > 450.0 {
            transform.translation.x = -450.0;
        } else if transform.translation.x < -450.0 {
            transform.translation.x = 450.0;
        }

        // Oscillate alpha
        wisp.alpha_phase += 0.5 * time.delta_secs();
        let base_alpha = 0.15 + 0.1 * wisp.alpha_phase.sin();

        // Adjust for weather transition
        let transition_alpha = if weather.current == WeatherType::Fog {
            if weather.target == WeatherType::Fog {
                1.0
            } else {
                1.0 - weather.transition_progress
            }
        } else {
            weather.transition_progress
        };

        sprite.color.set_alpha(base_alpha * transition_alpha);
    }
}
