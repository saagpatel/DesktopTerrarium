use bevy::prelude::*;
use crate::resources::{TimeOfDay, DebugSettings};
use crate::components::TimeVariantTag;

pub fn time_of_day_system(
    mut time_of_day: ResMut<TimeOfDay>,
    debug: Res<DebugSettings>,
    time: Res<Time>,
    mut sprites: Query<(&TimeVariantTag, &mut Sprite)>,
) {
    // Handle debug overrides
    if let Some(forced_phase) = debug.force_time_phase {
        // Force specific phase - show only that phase at full alpha
        for (tag, mut sprite) in &mut sprites {
            sprite.color.set_alpha(if tag.phase == forced_phase { 1.0 } else { 0.0 });
        }
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

    // Get crossfade parameters
    let (current_phase, next_phase, blend) = time_of_day.crossfade_params();

    // Apply crossfade to all time-variant sprites
    for (tag, mut sprite) in &mut sprites {
        let alpha = if tag.phase == current_phase {
            1.0 - blend
        } else if tag.phase == next_phase {
            blend
        } else {
            0.0
        };

        sprite.color.set_alpha(alpha);
    }
}
