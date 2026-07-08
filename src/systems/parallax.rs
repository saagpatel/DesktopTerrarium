use crate::components::SceneLayer;
use bevy::prelude::*;

const MAX_PARALLAX_PX: f32 = 15.0;
const SMOOTHING_SPEED: f32 = 5.0;

pub fn parallax_system(
    windows: Query<&Window>,
    mut layers: Query<(&SceneLayer, &mut Transform)>,
    time: Res<Time>,
) {
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let cursor_pos = window.cursor_position();
    let (width, height) = (window.width(), window.height());

    // Calculate target offset
    let target_offset = if let Some(pos) = cursor_pos {
        // Convert cursor position to offset from center
        let offset = pos - Vec2::new(width / 2.0, height / 2.0);
        // Normalize to -1..1 range
        let normalized = offset / Vec2::new(width / 2.0, height / 2.0);
        normalized * MAX_PARALLAX_PX
    } else {
        // Cursor outside window - drift back to center
        Vec2::ZERO
    };

    // Apply parallax to each layer with smoothing
    for (layer, mut transform) in &mut layers {
        let current_offset = Vec2::new(transform.translation.x, transform.translation.y);
        let layer_target = target_offset * layer.depth_factor;

        // Lerp toward target
        let new_offset = current_offset.lerp(layer_target, SMOOTHING_SPEED * time.delta_secs());

        transform.translation.x = new_offset.x;
        transform.translation.y = new_offset.y;
    }
}
