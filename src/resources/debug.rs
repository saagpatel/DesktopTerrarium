use crate::components::CritterSpecies;
use crate::resources::WeatherType;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityMode {
    System,
    ForceActive,
    ForceIdle,
}

impl ActivityMode {
    pub fn next(self) -> Self {
        match self {
            Self::System => Self::ForceActive,
            Self::ForceActive => Self::ForceIdle,
            Self::ForceIdle => Self::System,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::ForceActive => "forced-active",
            Self::ForceIdle => "forced-idle",
        }
    }
}

#[derive(Resource)]
pub struct DebugSettings {
    /// Show FPS counter and state overlay. Toggle with F1.
    pub show_overlay: bool,
    /// Time scale multiplier. 1.0 = normal, 10.0 = 10x fast forward. Affects time_of_day and weather.
    pub time_scale: f32,
    /// Multiplies plant growth for smoke testing.
    pub growth_rate_multiplier: f32,
    /// Force a specific weather type (overrides state machine). None = normal behavior.
    pub force_weather: Option<WeatherType>,
    /// Force a specific time-of-day phase. None = normal behavior.
    pub force_time_phase: Option<u8>,
    /// Override system activity tracking for deterministic smoke tests.
    pub activity_mode: ActivityMode,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            show_overlay: false,
            time_scale: 1.0,
            growth_rate_multiplier: 1.0,
            force_weather: None,
            force_time_phase: None,
            activity_mode: ActivityMode::System,
        }
    }
}

impl DebugSettings {
    pub fn cycle_time_scale(&mut self) {
        self.time_scale = match self.time_scale as i32 {
            1 => 5.0,
            5 => 20.0,
            20 => 100.0,
            _ => 1.0,
        };
    }

    pub fn cycle_growth_rate_multiplier(&mut self) {
        self.growth_rate_multiplier = match self.growth_rate_multiplier as i32 {
            1 => 25.0,
            25 => 250.0,
            _ => 1.0,
        };
    }
}

#[derive(Resource, Default, Debug)]
pub struct DebugActions {
    pub save_state: bool,
    pub advance_plants: bool,
    pub exit_after_save: bool,
    pub spawn_critter: Option<CritterSpecies>,
}

#[derive(Resource, Debug)]
pub struct DebugTelemetry {
    pub recent_events: VecDeque<String>,
    pub last_save_status: String,
    pub state_file_path: Option<String>,
}

impl Default for DebugTelemetry {
    fn default() -> Self {
        Self {
            recent_events: VecDeque::with_capacity(8),
            last_save_status: "not saved this session".to_string(),
            state_file_path: None,
        }
    }
}

impl DebugTelemetry {
    pub fn push_event(&mut self, message: impl Into<String>) {
        self.recent_events.push_front(message.into());
        while self.recent_events.len() > 8 {
            self.recent_events.pop_back();
        }
    }

    pub fn set_save_status(&mut self, status: impl Into<String>) {
        self.last_save_status = status.into();
    }
}

#[derive(Resource, Debug)]
pub struct SmokeScript {
    pub enabled: bool,
    pub step: usize,
    pub timer: Timer,
}

impl Default for SmokeScript {
    fn default() -> Self {
        let enabled = std::env::var("TERRARIUM_SMOKE_SCRIPT")
            .ok()
            .map(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes"
                )
            })
            .unwrap_or(false);

        Self {
            enabled,
            step: 0,
            timer: Timer::from_seconds(0.75, TimerMode::Repeating),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_mode_cycles_predictably() {
        assert_eq!(ActivityMode::System.next(), ActivityMode::ForceActive);
        assert_eq!(ActivityMode::ForceActive.next(), ActivityMode::ForceIdle);
        assert_eq!(ActivityMode::ForceIdle.next(), ActivityMode::System);
    }

    #[test]
    fn debug_settings_cycle_time_scale() {
        let mut settings = DebugSettings::default();
        settings.cycle_time_scale();
        assert_eq!(settings.time_scale, 5.0);
        settings.cycle_time_scale();
        assert_eq!(settings.time_scale, 20.0);
        settings.cycle_time_scale();
        assert_eq!(settings.time_scale, 100.0);
        settings.cycle_time_scale();
        assert_eq!(settings.time_scale, 1.0);
    }

    #[test]
    fn debug_settings_cycle_growth_multiplier() {
        let mut settings = DebugSettings::default();
        settings.cycle_growth_rate_multiplier();
        assert_eq!(settings.growth_rate_multiplier, 25.0);
        settings.cycle_growth_rate_multiplier();
        assert_eq!(settings.growth_rate_multiplier, 250.0);
        settings.cycle_growth_rate_multiplier();
        assert_eq!(settings.growth_rate_multiplier, 1.0);
    }
}
