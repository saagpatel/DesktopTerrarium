use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Plant {
    pub species: PlantSpecies,
    /// 0..=4. Stage 0 = seed, 4 = bloom.
    pub stage: u8,
    /// 0.0..1.0 progress within current stage. When >= 1.0, advance stage and reset to 0.0.
    /// At stage 4, progress is clamped at 1.0 (fully bloomed).
    pub growth_progress: f32,
    /// Position index in the terrarium (0, 1, 2 for the 3 plant slots).
    pub slot: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum PlantSpecies {
    Fern,
    Moss,
    Succulent,
}

impl PlantSpecies {
    pub fn asset_name(&self) -> &'static str {
        match self {
            PlantSpecies::Fern => "fern",
            PlantSpecies::Moss => "moss",
            PlantSpecies::Succulent => "succulent",
        }
    }
}
