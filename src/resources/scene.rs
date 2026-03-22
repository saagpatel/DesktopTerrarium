use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Resource)]
pub enum TerrariumScenePreset {
    #[default]
    ConservatoryDesk,
}

impl TerrariumScenePreset {
    pub fn label(self) -> &'static str {
        match self {
            Self::ConservatoryDesk => "conservatory-desk",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SceneMoodPreset {
    pub ambient_color: Color,
    pub ambient_brightness: f32,
    pub key_color: Color,
    pub key_illuminance: f32,
    pub fill_color: Color,
    pub fill_intensity: f32,
    pub rim_color: Color,
    pub rim_intensity: f32,
    pub backdrop_color: Color,
    pub backdrop_emissive: LinearRgba,
    pub haze_alpha: f32,
    pub wetness: f32,
}

#[derive(Resource)]
pub struct SceneMoodState {
    pub preset: TerrariumScenePreset,
    pub current: SceneMoodPreset,
}

impl Default for SceneMoodState {
    fn default() -> Self {
        Self {
            preset: TerrariumScenePreset::default(),
            current: SceneMoodPreset::day_clear(),
        }
    }
}

impl SceneMoodPreset {
    pub fn day_clear() -> Self {
        Self {
            ambient_color: Color::srgb(0.78, 0.84, 0.88),
            ambient_brightness: 115.0,
            key_color: Color::srgb(1.0, 0.94, 0.86),
            key_illuminance: 7_500.0,
            fill_color: Color::srgb(0.72, 0.82, 0.92),
            fill_intensity: 900.0,
            rim_color: Color::srgb(1.0, 0.98, 0.92),
            rim_intensity: 1_400.0,
            backdrop_color: Color::srgb(0.23, 0.27, 0.31),
            backdrop_emissive: LinearRgba::rgb(0.03, 0.03, 0.035),
            haze_alpha: 0.04,
            wetness: 0.08,
        }
    }

    pub fn for_phase_and_weather(phase: u8, wetness_boost: f32, haze_boost: f32) -> Self {
        let mut preset = match phase {
            0 => Self {
                ambient_color: Color::srgb(0.82, 0.78, 0.7),
                ambient_brightness: 90.0,
                key_color: Color::srgb(1.0, 0.78, 0.6),
                key_illuminance: 5_400.0,
                fill_color: Color::srgb(0.6, 0.7, 0.8),
                fill_intensity: 700.0,
                rim_color: Color::srgb(1.0, 0.9, 0.82),
                rim_intensity: 1_100.0,
                backdrop_color: Color::srgb(0.32, 0.27, 0.22),
                backdrop_emissive: LinearRgba::rgb(0.055, 0.04, 0.03),
                haze_alpha: 0.06,
                wetness: 0.12,
            },
            1 => Self::day_clear(),
            2 => Self {
                ambient_color: Color::srgb(0.64, 0.6, 0.7),
                ambient_brightness: 62.0,
                key_color: Color::srgb(0.98, 0.68, 0.5),
                key_illuminance: 3_900.0,
                fill_color: Color::srgb(0.44, 0.5, 0.62),
                fill_intensity: 540.0,
                rim_color: Color::srgb(0.95, 0.78, 0.72),
                rim_intensity: 1_000.0,
                backdrop_color: Color::srgb(0.19, 0.16, 0.2),
                backdrop_emissive: LinearRgba::rgb(0.035, 0.022, 0.028),
                haze_alpha: 0.08,
                wetness: 0.16,
            },
            _ => Self {
                ambient_color: Color::srgb(0.32, 0.38, 0.48),
                ambient_brightness: 28.0,
                key_color: Color::srgb(0.56, 0.68, 0.94),
                key_illuminance: 1_800.0,
                fill_color: Color::srgb(0.18, 0.26, 0.38),
                fill_intensity: 260.0,
                rim_color: Color::srgb(0.82, 0.88, 1.0),
                rim_intensity: 760.0,
                backdrop_color: Color::srgb(0.06, 0.08, 0.12),
                backdrop_emissive: LinearRgba::rgb(0.01, 0.013, 0.018),
                haze_alpha: 0.1,
                wetness: 0.2,
            },
        };

        preset.haze_alpha = (preset.haze_alpha + haze_boost).clamp(0.0, 0.25);
        preset.wetness = (preset.wetness + wetness_boost).clamp(0.0, 1.0);
        preset
    }
}
