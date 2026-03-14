use crate::resources::{FeatureToggles, WeatherState, WeatherType};
use bevy::prelude::*;

pub fn enforce_feature_fallbacks(mut weather: ResMut<WeatherState>, toggles: Res<FeatureToggles>) {
    if toggles.weather_forced_clear() {
        weather.current = WeatherType::Clear;
        weather.target = WeatherType::Clear;
        weather.transition_progress = 1.0;
        weather.phase_elapsed = 0.0;
    }
}
