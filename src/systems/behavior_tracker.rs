use crate::errors::TerrariumError;
use crate::resources::{ActivityMode, BehaviorSignals, DebugSettings};
use bevy::prelude::*;

const ACTIVE_IDLE_THRESHOLD_SECS: f64 = 120.0;

#[cfg(target_os = "macos")]
extern "C" {
    fn CGEventSourceSecondsSinceLastEventType(source_state_id: i32, event_type: u64) -> f64;
}

#[cfg(target_os = "macos")]
const K_CG_EVENT_SOURCE_STATE_COMBINED_SESSION_STATE: i32 = 0;
#[cfg(target_os = "macos")]
const K_CG_ANY_INPUT_EVENT_TYPE: u64 = u64::MAX;

#[cfg(target_os = "macos")]
fn get_system_idle_secs() -> Result<f64, TerrariumError> {
    let secs = unsafe {
        CGEventSourceSecondsSinceLastEventType(
            K_CG_EVENT_SOURCE_STATE_COMBINED_SESSION_STATE,
            K_CG_ANY_INPUT_EVENT_TYPE,
        )
    };
    if secs < 0.0 {
        Err(TerrariumError::IdleTimeQueryFailed(
            "negative value returned".into(),
        ))
    } else {
        Ok(secs)
    }
}

#[cfg(not(target_os = "macos"))]
fn get_system_idle_secs() -> Result<f64, TerrariumError> {
    // Keep behavior deterministic on unsupported targets: assume active instead of failing build.
    Ok(0.0)
}

fn update_behavior_counters(behavior: &mut BehaviorSignals, elapsed_secs: f64) {
    if elapsed_secs <= 0.0 {
        return;
    }

    if behavior.is_active {
        behavior.total_active_secs += elapsed_secs;
        behavior.current_focus_streak_secs += elapsed_secs;
        behavior.longest_focus_streak_secs = behavior
            .longest_focus_streak_secs
            .max(behavior.current_focus_streak_secs);
    } else if behavior.current_focus_streak_secs > 0.0 {
        behavior.current_focus_streak_secs = 0.0;
    }
}

pub fn behavior_tracker_system(
    mut behavior: ResMut<BehaviorSignals>,
    debug: Res<DebugSettings>,
    time: Res<Time>,
) {
    let elapsed_secs = time.delta_secs_f64();

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

    // Determine if user is active (idle < 120 seconds), unless a debug override is active.
    behavior.is_active = match debug.activity_mode {
        ActivityMode::System => behavior.system_idle_secs < ACTIVE_IDLE_THRESHOLD_SECS,
        ActivityMode::ForceActive => true,
        ActivityMode::ForceIdle => false,
    };
    update_behavior_counters(&mut behavior, elapsed_secs);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(lhs: f64, rhs: f64) {
        assert!((lhs - rhs).abs() < 1e-9, "left={lhs}, right={rhs}");
    }

    #[test]
    fn active_time_accumulates_by_elapsed_seconds() {
        let mut behavior = BehaviorSignals {
            is_active: true,
            ..Default::default()
        };

        update_behavior_counters(&mut behavior, 0.5);
        update_behavior_counters(&mut behavior, 1.25);

        approx_eq(behavior.total_active_secs, 1.75);
        approx_eq(behavior.current_focus_streak_secs, 1.75);
        approx_eq(behavior.longest_focus_streak_secs, 1.75);
    }

    #[test]
    fn inactive_transition_resets_current_streak_only() {
        let mut behavior = BehaviorSignals {
            is_active: true,
            ..Default::default()
        };
        update_behavior_counters(&mut behavior, 10.0);

        behavior.is_active = false;
        update_behavior_counters(&mut behavior, 0.2);

        approx_eq(behavior.total_active_secs, 10.0);
        approx_eq(behavior.current_focus_streak_secs, 0.0);
        approx_eq(behavior.longest_focus_streak_secs, 10.0);
    }

    #[test]
    fn non_positive_elapsed_time_does_not_change_counters() {
        let mut behavior = BehaviorSignals {
            is_active: true,
            ..Default::default()
        };

        update_behavior_counters(&mut behavior, 0.0);
        update_behavior_counters(&mut behavior, -1.0);

        approx_eq(behavior.total_active_secs, 0.0);
        approx_eq(behavior.current_focus_streak_secs, 0.0);
        approx_eq(behavior.longest_focus_streak_secs, 0.0);
    }
}
