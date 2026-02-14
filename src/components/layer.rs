use bevy::prelude::*;

/// Marker + config for a scene background layer that participates in parallax + time-of-day crossfade.
#[derive(Component)]
pub struct SceneLayer {
    /// Parallax depth factor. 0.0 = no movement, 1.0 = moves 1:1 with mouse offset.
    /// Background layers ~0.3, midground ~0.6, foreground ~1.0.
    pub depth_factor: f32,
    /// Which time-of-day variant set this layer belongs to.
    /// None if the layer doesn't change with time (e.g., decorations).
    pub time_variant: Option<TimeVariantSet>,
}

/// Groups the 4 sprite entities for a single layer's time-of-day variants.
/// All 4 are always spawned; we crossfade by adjusting their alpha.
#[derive(Clone)]
pub struct TimeVariantSet {
    pub morning: Entity,
    pub day: Entity,
    pub evening: Entity,
    pub night: Entity,
}

#[derive(Component)]
pub struct TimeVariantTag {
    pub layer_id: u8,  // e.g., 0 = glass_container, 1 = soil
    pub phase: u8,     // 0=morning, 1=day, 2=evening, 3=night
}
