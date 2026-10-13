use crate::errors::TerrariumError;
use crate::resources::{ActivityStatus, BehaviorSignals};
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
    } else if !secs.is_finite() {
        Err(TerrariumError::IdleTimeQueryFailed(
            "non-finite value returned".into(),
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

    if behavior.activity == ActivityStatus::Active {
        behavior.total_active_secs += elapsed_secs;
        behavior.current_focus_streak_secs += elapsed_secs;
        behavior.longest_focus_streak_secs = behavior
            .longest_focus_streak_secs
            .max(behavior.current_focus_streak_secs);
    } else if behavior.current_focus_streak_secs > 0.0 {
        behavior.current_focus_streak_secs = 0.0;
    }
}

fn apply_idle_outcome(behavior: &mut BehaviorSignals, outcome: Result<f64, TerrariumError>) {
    match outcome {
        Ok(idle_secs) if idle_secs.is_finite() && idle_secs >= 0.0 => {
            behavior.system_idle_secs = idle_secs;
            behavior.activity = if idle_secs < ACTIVE_IDLE_THRESHOLD_SECS {
                ActivityStatus::Active
            } else {
                ActivityStatus::Idle
            };
        }
        _ => behavior.activity = ActivityStatus::Unavailable,
    }
}

fn update_behavior_from_idle_outcome(
    behavior: &mut BehaviorSignals,
    outcome: Result<f64, TerrariumError>,
    elapsed_secs: f64,
) {
    apply_idle_outcome(behavior, outcome);
    if behavior.activity != ActivityStatus::Unavailable {
        update_behavior_counters(behavior, elapsed_secs);
    }
}

pub fn behavior_tracker_system(
    mut behavior: ResMut<BehaviorSignals>,
    time: Res<Time>,
    mut previous_status: Local<Option<ActivityStatus>>,
) {
    let elapsed_secs = time.delta_secs_f64();

    update_behavior_from_idle_outcome(&mut behavior, get_system_idle_secs(), elapsed_secs);
    if previous_status.is_none_or(|status| status != behavior.activity) {
        match behavior.activity {
            ActivityStatus::Unavailable => {
                warn!("System idle time is unavailable; pausing activity growth")
            }
            ActivityStatus::Idle => info!("System activity state changed to idle"),
            ActivityStatus::Active => info!("System activity state changed to active"),
        }
        *previous_status = Some(behavior.activity);
    }
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
            activity: ActivityStatus::Active,
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
            activity: ActivityStatus::Active,
            ..Default::default()
        };
        update_behavior_counters(&mut behavior, 10.0);

        behavior.activity = ActivityStatus::Idle;
        update_behavior_counters(&mut behavior, 0.2);

        approx_eq(behavior.total_active_secs, 10.0);
        approx_eq(behavior.current_focus_streak_secs, 0.0);
        approx_eq(behavior.longest_focus_streak_secs, 10.0);
    }

    #[test]
    fn non_positive_elapsed_time_does_not_change_counters() {
        let mut behavior = BehaviorSignals {
            activity: ActivityStatus::Active,
            ..Default::default()
        };

        update_behavior_counters(&mut behavior, 0.0);
        update_behavior_counters(&mut behavior, -1.0);

        approx_eq(behavior.total_active_secs, 0.0);
        approx_eq(behavior.current_focus_streak_secs, 0.0);
        approx_eq(behavior.longest_focus_streak_secs, 0.0);
    }

    #[test]
    fn unavailable_and_invalid_idle_values_award_no_focus_time() {
        for outcome in [
            Err(TerrariumError::IdleTimeQueryFailed("injected".into())),
            Ok(f64::NAN),
            Ok(f64::INFINITY),
            Ok(-1.0),
        ] {
            let mut behavior = BehaviorSignals {
                activity: ActivityStatus::Active,
                current_focus_streak_secs: 12.0,
                ..Default::default()
            };
            update_behavior_from_idle_outcome(&mut behavior, outcome, 1.0);
            assert_eq!(behavior.activity, ActivityStatus::Unavailable);
            approx_eq(behavior.total_active_secs, 0.0);
            approx_eq(behavior.current_focus_streak_secs, 12.0);
        }
    }
}
