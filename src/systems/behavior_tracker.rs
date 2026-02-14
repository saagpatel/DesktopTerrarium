use bevy::prelude::*;
use crate::resources::BehaviorSignals;
use crate::errors::TerrariumError;

extern "C" {
    fn CGEventSourceSecondsSinceLastEventType(
        source_state_id: i32,
        event_type: u64,
    ) -> f64;
}

const K_CG_EVENT_SOURCE_STATE_COMBINED_SESSION_STATE: i32 = 0;
const K_CG_ANY_INPUT_EVENT_TYPE: u64 = u64::MAX;

fn get_system_idle_secs() -> Result<f64, TerrariumError> {
    let secs = unsafe {
        CGEventSourceSecondsSinceLastEventType(
            K_CG_EVENT_SOURCE_STATE_COMBINED_SESSION_STATE,
            K_CG_ANY_INPUT_EVENT_TYPE,
        )
    };
    if secs < 0.0 {
        Err(TerrariumError::IdleTimeQueryFailed("negative value returned".into()))
    } else {
        Ok(secs)
    }
}

pub fn behavior_tracker_system(
    mut behavior: ResMut<BehaviorSignals>,
) {
    // Poll system idle time
    match get_system_idle_secs() {
        Ok(idle_secs) => {
            behavior.system_idle_secs = idle_secs;
        }
        Err(e) => {
            warn!("Failed to get system idle time: {}", e);
            behavior.system_idle_secs = 0.0; // Assume active on error
        }
    }

    // Determine if user is active (idle < 120 seconds)
    behavior.is_active = behavior.system_idle_secs < 120.0;

    // Update activity counters
    if behavior.is_active {
        behavior.total_active_secs += 1.0;
        behavior.current_focus_streak_secs += 1.0;
    } else if behavior.current_focus_streak_secs > 0.0 {
        // Just became inactive - update longest streak if needed
        if behavior.current_focus_streak_secs > behavior.longest_focus_streak_secs {
            behavior.longest_focus_streak_secs = behavior.current_focus_streak_secs;
        }
        behavior.current_focus_streak_secs = 0.0;
    }
}
