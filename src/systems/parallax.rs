use crate::components::MainTerrariumCamera;
use bevy::prelude::*;

const CAMERA_SWAY_X: f32 = 0.18;
const CAMERA_SWAY_Y: f32 = 0.12;
const CAMERA_IDLE_BOB: f32 = 0.06;
const CAMERA_SMOOTHING: f32 = 4.0;

pub fn parallax_system(
    time: Res<Time>,
    windows: Query<&Window>,
    mut camera: Query<(&MainTerrariumCamera, &mut Transform)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((rig, mut transform)) = camera.get_single_mut() else {
        return;
    };

    let cursor_offset = window
        .cursor_position()
        .map(|pos| {
            let offset = pos - Vec2::new(window.width() / 2.0, window.height() / 2.0);
            offset / Vec2::new(window.width() / 2.0, window.height() / 2.0)
        })
        .unwrap_or(Vec2::ZERO);

    let idle_offset = Vec3::new(
        (time.elapsed_secs() * 0.35).sin() * 0.04,
        (time.elapsed_secs() * 0.7).sin() * CAMERA_IDLE_BOB,
        0.0,
    );

    let target_translation = rig.base_translation
        + Vec3::new(
            cursor_offset.x * CAMERA_SWAY_X,
            cursor_offset.y * CAMERA_SWAY_Y,
            cursor_offset.x.abs() * 0.05,
        )
        + idle_offset;

    transform.translation = transform
        .translation
        .lerp(target_translation, CAMERA_SMOOTHING * time.delta_secs());
    transform.look_at(rig.target, Vec3::Y);
}
