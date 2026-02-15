use crate::events::WeatherChanged;
use crate::resources::{DebugSettings, WeatherState};
use bevy::prelude::*;

pub fn weather_transition_system(
    mut weather: ResMut<WeatherState>,
    debug: Res<DebugSettings>,
    time: Res<Time>,
    mut events: EventWriter<WeatherChanged>,
) {
    // Handle debug overrides
    if let Some(forced_weather) = debug.force_weather {
        weather.current = forced_weather;
        weather.target = forced_weather;
        weather.transition_progress = 1.0;
        return;
    }

    // Advance time in current phase
    weather.phase_elapsed += time.delta_secs() * debug.time_scale;

    // Check if we're currently transitioning
    if weather.transition_progress < 1.0 {
        // Continue transition
        weather.transition_progress += time.delta_secs() / weather.transition_duration_secs;

        if weather.transition_progress >= 1.0 {
            // Transition complete
            weather.current = weather.target;
            weather.transition_progress = 1.0;
        }
    } else {
        // Not transitioning, check if it's time to start a new transition
        if weather.phase_elapsed >= weather.phase_duration_secs {
            // Start transition to next weather
            let new_target = weather.current.next();
            events.send(WeatherChanged {
                from: weather.current,
                to: new_target,
            });

            weather.target = new_target;
            weather.transition_progress = 0.0;
            weather.phase_elapsed = 0.0;
        }
    }
}
