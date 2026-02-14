use bevy::prelude::*;
use crate::resources::WeatherType;

#[derive(Resource)]
pub struct DebugSettings {
    /// Show FPS counter and state overlay. Toggle with F1.
    pub show_overlay: bool,
    /// Time scale multiplier. 1.0 = normal, 10.0 = 10x fast forward. Affects time_of_day and weather.
    pub time_scale: f32,
    /// Force a specific weather type (overrides state machine). None = normal behavior.
    pub force_weather: Option<WeatherType>,
    /// Force a specific time-of-day phase. None = normal behavior.
    pub force_time_phase: Option<u8>,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            show_overlay: false,
            time_scale: 1.0,
            force_weather: None,
            force_time_phase: None,
        }
    }
}
