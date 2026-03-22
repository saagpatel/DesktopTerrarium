use crate::components::FogWisp;
use crate::resources::{FeatureToggles, WeatherState, WeatherType};
use crate::systems::setup::SceneAssetHandles;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct FogAssets;

pub fn setup_fog_assets(mut commands: Commands) {
    commands.insert_resource(FogAssets);
}

pub fn fog_spawn_system(
    mut commands: Commands,
    _fog_assets: Res<FogAssets>,
    scene_assets: Res<SceneAssetHandles>,
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
            Mesh3d(scene_assets.sphere_mesh.clone()),
            MeshMaterial3d(scene_assets.fog_material.clone()),
            Transform::from_xyz(
                rng.gen_range(-2.6..2.6),
                rng.gen_range(0.35..2.3),
                rng.gen_range(-1.7..1.8),
            )
            .with_scale(Vec3::new(
                rng.gen_range(0.45..0.9),
                rng.gen_range(0.18..0.35),
                rng.gen_range(0.45..0.9),
            )),
            FogWisp {
                drift_speed: rng.gen_range(0.05..0.18),
                alpha_phase: rng.gen_range(0.0..std::f32::consts::TAU),
            },
        ));
    }
}

pub fn fog_update_system(
    mut commands: Commands,
    toggles: Res<FeatureToggles>,
    mut fog: Query<(
        Entity,
        &mut FogWisp,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    weather: Res<WeatherState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if !toggles.weather_particles_enabled() {
        for (entity, _, _, _) in &mut fog {
            commands.entity(entity).despawn();
        }
        return;
    }

    let should_have_fog = weather.current == WeatherType::Fog || weather.target == WeatherType::Fog;

    for (entity, mut wisp, mut transform, material) in &mut fog {
        if !should_have_fog {
            commands.entity(entity).despawn();
            continue;
        }

        // Drift and softly bob across the vessel.
        transform.translation.x += wisp.drift_speed * time.delta_secs();
        transform.translation.z += (wisp.alpha_phase * 0.7).sin() * 0.03 * time.delta_secs();
        transform.translation.y += (wisp.alpha_phase * 1.1).cos() * 0.02 * time.delta_secs();

        // Wrap around terrarium width.
        if transform.translation.x > 2.8 {
            transform.translation.x = -2.8;
        } else if transform.translation.x < -2.8 {
            transform.translation.x = 2.8;
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

        if let Some(material) = materials.get_mut(&material.0) {
            material.base_color = Color::srgba(
                0.88,
                0.93,
                0.97,
                (base_alpha * transition_alpha).clamp(0.02, 0.24),
            );
        }
    }
}
