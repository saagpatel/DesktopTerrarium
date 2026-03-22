use crate::components::{HazeVolume, LightRigRole, WindReactive};
use crate::resources::{SceneMoodPreset, SceneMoodState, TimeOfDay, WeatherState, WeatherType};
use crate::systems::setup::SceneAssetHandles;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct SceneMoodParams<'w, 's> {
    assets: Res<'w, SceneAssetHandles>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    scene_mood: ResMut<'w, SceneMoodState>,
    haze: Query<'w, 's, &'static mut Transform, With<HazeVolume>>,
    point_lights: Query<'w, 's, (&'static LightRigRole, &'static mut PointLight)>,
    directional_lights: Query<'w, 's, (&'static LightRigRole, &'static mut DirectionalLight)>,
}

pub fn scene_mood_system(
    mut ambient_light: ResMut<AmbientLight>,
    mut scene: SceneMoodParams,
    time_of_day: Res<TimeOfDay>,
    weather: Res<WeatherState>,
) {
    let wetness_boost = match weather.current {
        WeatherType::Rain => 0.32,
        WeatherType::Fog => 0.12,
        _ => 0.0,
    };
    let haze_boost = match weather.current {
        WeatherType::Fog => 0.1,
        WeatherType::Rain => 0.04,
        _ => 0.0,
    };

    let preset =
        SceneMoodPreset::for_phase_and_weather(time_of_day.phase, wetness_boost, haze_boost);
    scene.scene_mood.current = preset;

    ambient_light.color = preset.ambient_color;
    ambient_light.brightness = preset.ambient_brightness;

    for (role, mut light) in &mut scene.directional_lights {
        if *role == LightRigRole::Key {
            light.color = preset.key_color;
            light.illuminance = preset.key_illuminance;
        }
    }

    for (role, mut light) in &mut scene.point_lights {
        match role {
            LightRigRole::Fill => {
                light.color = preset.fill_color;
                light.intensity = preset.fill_intensity;
            }
            LightRigRole::Rim => {
                light.color = preset.rim_color;
                light.intensity = preset.rim_intensity;
            }
            LightRigRole::Key => {}
        }
    }

    if let Some(backdrop) = scene.materials.get_mut(&scene.assets.backdrop_material) {
        backdrop.base_color = preset.backdrop_color;
        backdrop.emissive = preset.backdrop_emissive;
    }

    if let Some(glass) = scene.materials.get_mut(&scene.assets.glass_material) {
        glass.base_color = Color::srgba(
            0.84 + preset.wetness * 0.04,
            0.93 + preset.wetness * 0.03,
            0.97 + preset.wetness * 0.02,
            0.16 + preset.wetness * 0.08,
        );
        glass.reflectance = 0.9 + preset.wetness * 0.06;
        glass.perceptual_roughness = (0.08 - preset.wetness * 0.03).clamp(0.04, 0.12);
    }

    if let Some(soil) = scene.materials.get_mut(&scene.assets.soil_material) {
        soil.base_color = Color::srgb(
            0.31 - preset.wetness * 0.08,
            0.2 - preset.wetness * 0.04,
            0.12 - preset.wetness * 0.02,
        );
        soil.perceptual_roughness = (0.95 - preset.wetness * 0.25).clamp(0.55, 0.95);
    }

    if let Some(moss) = scene.materials.get_mut(&scene.assets.moss_material) {
        moss.base_color = Color::srgb(
            0.22 - preset.wetness * 0.02,
            0.33 + preset.wetness * 0.06,
            0.18 - preset.wetness * 0.01,
        );
    }

    if let Some(highlight) = scene
        .materials
        .get_mut(&scene.assets.glass_highlight_material)
    {
        highlight.base_color = Color::srgba(0.97, 0.99, 1.0, 0.14 + preset.wetness * 0.08);
    }

    if let Some(fog) = scene.materials.get_mut(&scene.assets.fog_material) {
        fog.base_color = Color::srgba(0.86, 0.92, 0.96, preset.haze_alpha);
    }

    for mut haze_transform in &mut scene.haze {
        haze_transform.scale = Vec3::new(
            6.4 + preset.haze_alpha * 2.0,
            3.0 + preset.haze_alpha * 4.0,
            3.6 + preset.haze_alpha * 2.0,
        );
    }
}

pub fn wind_reactive_system(
    mut query: Query<(&WindReactive, &mut Transform), Without<HazeVolume>>,
    time: Res<Time>,
    weather: Res<WeatherState>,
) {
    let intensity = match weather.current {
        WeatherType::Wind => 1.0,
        WeatherType::Rain => 0.35,
        _ => 0.12,
    };

    for (reactive, mut transform) in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            (time.elapsed_secs() * 0.8 + reactive.phase_offset).sin()
                * reactive.amplitude
                * intensity,
            0.0,
            (time.elapsed_secs() * 0.45 + reactive.phase_offset).sin()
                * reactive.amplitude
                * 0.45
                * intensity,
        );
    }
}
