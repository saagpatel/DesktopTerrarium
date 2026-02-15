use crate::components::PlantSpecies;
use crate::resources::WeatherType;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct TerrariumPersistentState {
    /// Schema version for forward compatibility. Current: 1.
    pub version: u32,
    /// When the terrarium was first created.
    pub created_at: String, // ISO 8601 string, avoids chrono in save format
    /// State of each plant slot.
    pub plants: [PlantSaveData; 3],
    /// Accumulated active seconds (carried across sessions).
    pub total_active_secs: f64,
    /// Longest focus streak ever.
    pub longest_focus_streak_secs: f64,
    /// Current weather type at save time.
    pub weather: WeatherType,
    /// Current time-of-day phase at save time.
    pub time_of_day_phase: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlantSaveData {
    pub species: PlantSpecies,
    pub stage: u8,
    pub growth_progress: f32,
}

impl Default for TerrariumPersistentState {
    fn default() -> Self {
        Self {
            version: 1,
            created_at: chrono::Utc::now().to_rfc3339(),
            plants: [
                PlantSaveData {
                    species: PlantSpecies::Fern,
                    stage: 0,
                    growth_progress: 0.0,
                },
                PlantSaveData {
                    species: PlantSpecies::Moss,
                    stage: 0,
                    growth_progress: 0.0,
                },
                PlantSaveData {
                    species: PlantSpecies::Succulent,
                    stage: 0,
                    growth_progress: 0.0,
                },
            ],
            total_active_secs: 0.0,
            longest_focus_streak_secs: 0.0,
            weather: WeatherType::Clear,
            time_of_day_phase: 1,
        }
    }
}
