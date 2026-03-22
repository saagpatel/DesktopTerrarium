use crate::resources::{DebugSettings, TimeOfDay};
use bevy::prelude::*;

pub fn time_of_day_system(
    mut time_of_day: ResMut<TimeOfDay>,
    debug: Res<DebugSettings>,
    time: Res<Time>,
) {
    // Handle debug overrides
    if let Some(forced_phase) = debug.force_time_phase {
        time_of_day.phase = forced_phase % 4;
        time_of_day.progress = 0.0;
        return;
    }

    // Advance time
    let phase_duration = time_of_day.cycle_duration_secs / 4.0;
    time_of_day.progress += (time.delta_secs() * debug.time_scale) / phase_duration;

    // Check if we need to advance to next phase
    if time_of_day.progress >= 1.0 {
        time_of_day.phase = (time_of_day.phase + 1) % 4;
        time_of_day.progress = 0.0;
    }
}
