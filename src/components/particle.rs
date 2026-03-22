use bevy::prelude::*;

#[derive(Component)]
pub struct RainDrop {
    pub velocity: Vec3,
    pub lifetime: f32, // seconds remaining
}

#[derive(Component)]
pub struct FogWisp {
    pub drift_speed: f32, // horizontal pixels per second
    pub alpha_phase: f32, // radians, for sine-wave alpha oscillation
}

#[derive(Component)]
pub struct WindLeaf {
    pub velocity: Vec3,
    pub rotation_speed: f32, // radians per second
    pub lifetime: f32,
}
