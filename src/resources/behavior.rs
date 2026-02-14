use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BehaviorSignals {
    /// Seconds the system has been idle (no keyboard/mouse). Polled from IOKit.
    pub system_idle_secs: f64,
    /// Accumulated seconds of "active" time (idle < 120s) since app launch.
    /// Persisted across sessions via TerrariumPersistentState.
    pub total_active_secs: f64,
    /// Current focus streak: consecutive active seconds without a break > 300s.
    pub current_focus_streak_secs: f64,
    /// Longest focus streak ever recorded. Persisted.
    pub longest_focus_streak_secs: f64,
    /// Whether the user is currently "active" (idle < 120 seconds).
    pub is_active: bool,
}
