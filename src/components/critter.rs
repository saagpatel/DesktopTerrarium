use crate::components::CritterPathId;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Critter {
    pub species: CritterSpecies,
    /// 0.0..1.0 progress along the Bezier path. When >= 1.0, despawn.
    pub path_progress: f32,
    /// Speed multiplier (units of path_progress per second). Default 0.05 = 20 second traversal.
    pub speed: f32,
    /// Control points for cubic Bezier curve (4 points).
    pub path: [Vec3; 4],
    pub path_id: CritterPathId,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum CritterSpecies {
    Butterfly,
    Beetle,
}
