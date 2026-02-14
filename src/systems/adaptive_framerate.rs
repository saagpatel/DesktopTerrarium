use bevy::prelude::*;
use bevy::winit::{WinitSettings, UpdateMode};
use std::time::Duration;

pub fn setup_framerate_limits(mut commands: Commands) {
    commands.insert_resource(WinitSettings {
        focused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(1.0 / 30.0)),
        unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(0.5)),
    });
}
