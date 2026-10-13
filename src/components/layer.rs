use bevy::prelude::*;

/// Marker + config for a scene background layer that participates in parallax.
#[derive(Component)]
pub struct SceneLayer {
    /// Parallax depth factor. 0.0 = no movement, 1.0 = moves 1:1 with mouse offset.
    /// Background layers ~0.3, midground ~0.6, foreground ~1.0.
    pub depth_factor: f32,
}

#[derive(Component)]
pub struct TimeVariantTag {
    pub layer_id: u8, // e.g., 0 = glass_container, 1 = soil
    pub phase: u8,    // 0=morning, 1=day, 2=evening, 3=night
}
