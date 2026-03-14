use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherExecutionMode {
    Full,
    Reduced,
    SafeDisable,
    StaticBaseline,
}

impl WeatherExecutionMode {
    fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "full" => Some(Self::Full),
            "reduced" => Some(Self::Reduced),
            "safe_disable" | "safe-disable" | "disable" => Some(Self::SafeDisable),
            "static_baseline" | "static-baseline" | "baseline" => Some(Self::StaticBaseline),
            _ => None,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct FeatureToggles {
    pub weather_mode: WeatherExecutionMode,
}

impl FeatureToggles {
    pub fn weather_transitions_enabled(self) -> bool {
        matches!(
            self.weather_mode,
            WeatherExecutionMode::Full | WeatherExecutionMode::Reduced
        )
    }

    pub fn weather_particles_enabled(self) -> bool {
        matches!(self.weather_mode, WeatherExecutionMode::Full)
    }

    pub fn weather_forced_clear(self) -> bool {
        matches!(
            self.weather_mode,
            WeatherExecutionMode::SafeDisable | WeatherExecutionMode::StaticBaseline
        )
    }
}

impl Default for FeatureToggles {
    fn default() -> Self {
        let weather_mode = std::env::var("TERRARIUM_WEATHER_MODE")
            .ok()
            .and_then(|raw| WeatherExecutionMode::parse(&raw))
            .unwrap_or(WeatherExecutionMode::Full);

        Self { weather_mode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known_weather_modes() {
        assert_eq!(
            WeatherExecutionMode::parse("full"),
            Some(WeatherExecutionMode::Full)
        );
        assert_eq!(
            WeatherExecutionMode::parse("safe_disable"),
            Some(WeatherExecutionMode::SafeDisable)
        );
        assert_eq!(
            WeatherExecutionMode::parse("STATIC-BASELINE"),
            Some(WeatherExecutionMode::StaticBaseline)
        );
    }

    #[test]
    fn toggles_expose_expected_capabilities() {
        let full = FeatureToggles {
            weather_mode: WeatherExecutionMode::Full,
        };
        assert!(full.weather_transitions_enabled());
        assert!(full.weather_particles_enabled());
        assert!(!full.weather_forced_clear());

        let reduced = FeatureToggles {
            weather_mode: WeatherExecutionMode::Reduced,
        };
        assert!(reduced.weather_transitions_enabled());
        assert!(!reduced.weather_particles_enabled());
        assert!(!reduced.weather_forced_clear());

        let disabled = FeatureToggles {
            weather_mode: WeatherExecutionMode::SafeDisable,
        };
        assert!(!disabled.weather_transitions_enabled());
        assert!(!disabled.weather_particles_enabled());
        assert!(disabled.weather_forced_clear());
    }
}
