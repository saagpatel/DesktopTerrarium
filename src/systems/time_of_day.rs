use crate::components::TimeVariantTag;
use crate::resources::TimeOfDay;
use bevy::prelude::*;

pub fn time_of_day_system(
    mut time_of_day: ResMut<TimeOfDay>,
    time: Res<Time>,
    mut sprites: Query<(&TimeVariantTag, &mut Sprite)>,
) {
    // Advance time
    let phase_duration = time_of_day.cycle_duration_secs / 4.0;
    time_of_day.progress += time.delta_secs() / phase_duration;

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
