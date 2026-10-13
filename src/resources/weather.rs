use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct WeatherState {
    pub current: WeatherType,
    pub target: WeatherType,
    /// 0.0..1.0. When transitioning, this ramps from 0 to 1. At 1.0, current = target.
    pub transition_progress: f32,
    /// Seconds per weather phase before transitioning to next.
    pub phase_duration_secs: f32,
    /// Seconds elapsed in current phase.
    pub phase_elapsed: f32,
    /// Transition speed: seconds for a full 0->1 transition. Default 30.0 (30 sec fade).
    pub transition_duration_secs: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum WeatherType {
    Clear,
    Fog,
    Rain,
    Wind,
}

pub fn weather_transition_factor(
    current: WeatherType,
    target: WeatherType,
    progress: f32,
    particle_weather: WeatherType,
) -> f32 {
    if current == particle_weather {
        if target == particle_weather {
            1.0
        } else {
            1.0 - progress.clamp(0.0, 1.0)
        }
    } else if target == particle_weather {
        progress.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

impl WeatherType {
    /// Deterministic next weather in the cycle.
    pub fn next(self) -> Self {
        match self {
            Self::Clear => Self::Fog,
            Self::Fog => Self::Rain,
            Self::Rain => Self::Wind,
            Self::Wind => Self::Clear,
        }
    }
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            current: WeatherType::Clear,
            target: WeatherType::Clear,
            transition_progress: 1.0,   // fully settled
            phase_duration_secs: 300.0, // 5 minutes per weather phase
            phase_elapsed: 0.0,
            transition_duration_secs: 30.0,
        }
    }
}
