use crate::resources::DebugSettings;
use bevy::prelude::*;

pub fn debug_input_system(keyboard: Res<ButtonInput<KeyCode>>, mut debug: ResMut<DebugSettings>) {
    // F1: Toggle overlay
    if keyboard.just_pressed(KeyCode::F1) {
        debug.show_overlay = !debug.show_overlay;
    }

    // F2: Cycle time phase
    if keyboard.just_pressed(KeyCode::F2) {
        debug.force_time_phase = match debug.force_time_phase {
            None => Some(0),
            Some(0) => Some(1),
            Some(1) => Some(2),
            Some(2) => Some(3),
            Some(3) => None,
            _ => None,
        };
    }

    // F4: Cycle weather
    if keyboard.just_pressed(KeyCode::F4) {
        use crate::resources::WeatherType;
        debug.force_weather = match debug.force_weather {
            None => Some(WeatherType::Clear),
            Some(WeatherType::Clear) => Some(WeatherType::Fog),
            Some(WeatherType::Fog) => Some(WeatherType::Rain),
            Some(WeatherType::Rain) => Some(WeatherType::Wind),
            Some(WeatherType::Wind) => None,
        };
    }

    // F5: Cycle time scale
    if keyboard.just_pressed(KeyCode::F5) {
        debug.time_scale = match debug.time_scale as i32 {
            1 => 5.0,
            5 => 20.0,
            20 => 100.0,
            _ => 1.0,
        };
    }
}
