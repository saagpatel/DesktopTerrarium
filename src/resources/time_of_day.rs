use bevy::prelude::*;

#[derive(Resource)]
pub struct TimeOfDay {
    /// Current phase: Morning(0), Day(1), Evening(2), Night(3)
    pub phase: u8,
    /// 0.0..1.0 progress through current phase. At 1.0, advance to next phase.
    pub progress: f32,
    /// Real-world seconds per full day cycle. Default: 1440.0 (24 minutes = 1 day).
    /// Debug override can set this to e.g. 60.0 for a 1-minute day.
    pub cycle_duration_secs: f32,
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            phase: 1, // start at Day
            progress: 0.0,
            cycle_duration_secs: 1440.0,
        }
    }
}

impl TimeOfDay {
    /// Returns the name of the current phase for asset loading.
    pub fn phase_name(&self) -> &'static str {
        match self.phase {
            0 => "morning",
            1 => "day",
            2 => "evening",
            3 => "night",
            _ => unreachable!(), // phase is always mod 4
        }
    }

    /// Returns (current_phase, next_phase, blend_factor) for crossfading.
    /// blend_factor is 0.0 at phase start (show current fully) and approaches 1.0
    /// in the last 20% of the phase (fade toward next).
    pub fn crossfade_params(&self) -> (u8, u8, f32) {
        let next = (self.phase + 1) % 4;
        let blend = if self.progress > 0.8 {
            (self.progress - 0.8) / 0.2 // ramp from 0..1 over last 20%
        } else {
            0.0
        };
        (self.phase, next, blend)
    }
}
