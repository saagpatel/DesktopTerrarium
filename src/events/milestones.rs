use crate::components::{CritterSpecies, PlantSpecies};
use crate::resources::WeatherType;
use bevy::prelude::*;

#[derive(Event)]
pub struct PlantStageChanged {
    pub entity: Entity,
    pub species: PlantSpecies,
    pub old_stage: u8,
    pub new_stage: u8,
}

#[derive(Event)]
pub struct CritterArrived {
    pub species: CritterSpecies,
}

#[derive(Event)]
pub struct CritterDeparted {
    pub species: CritterSpecies,
}

#[derive(Event)]
pub struct WeatherChanged {
    pub from: WeatherType,
    pub to: WeatherType,
}
