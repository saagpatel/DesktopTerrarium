use crate::components::Plant;
use crate::events::PlantStageChanged;
use crate::resources::{BehaviorSignals, DebugActions, DebugSettings, SceneArtCatalog};
use crate::systems::setup::{respawn_plant_visual, SceneAssetHandles};
use bevy::prelude::*;

const GROWTH_RATE_PER_SEC: f32 = 0.001; // ~17 minutes per stage

pub fn plant_growth_system(
    mut plants: Query<(Entity, &mut Plant)>,
    behavior: Res<BehaviorSignals>,
    debug: Res<DebugSettings>,
    mut events: EventWriter<PlantStageChanged>,
    time: Res<Time>,
) {
    let growth_delta = if behavior.is_active {
        GROWTH_RATE_PER_SEC * debug.growth_rate_multiplier * time.delta_secs()
    } else {
        0.0
    };

    for (entity, mut plant) in &mut plants {
        if plant.stage >= 4 {
            // Fully bloomed, clamp progress
            plant.growth_progress = plant.growth_progress.min(1.0);
            continue;
        }

        plant.growth_progress += growth_delta;

        // Check if we should advance to next stage
        if plant.growth_progress >= 1.0 {
            let old_stage = plant.stage;
            plant.stage += 1;
            plant.growth_progress = 0.0;

            events.send(PlantStageChanged {
                entity,
                species: plant.species,
                old_stage,
                new_stage: plant.stage,
            });
        }
    }
}

pub fn debug_advance_plants_system(
    mut actions: ResMut<DebugActions>,
    mut plants: Query<(Entity, &mut Plant)>,
    mut events: EventWriter<PlantStageChanged>,
) {
    if !actions.advance_plants {
        return;
    }
    actions.advance_plants = false;

    for (entity, mut plant) in &mut plants {
        if plant.stage >= 4 {
            plant.growth_progress = 1.0;
            continue;
        }

        let old_stage = plant.stage;
        plant.stage += 1;
        plant.growth_progress = 0.0;

        events.send(PlantStageChanged {
            entity,
            species: plant.species,
            old_stage,
            new_stage: plant.stage,
        });
    }
}

pub fn handle_plant_stage_changes(
    mut commands: Commands,
    assets: Res<SceneAssetHandles>,
    art_catalog: Res<SceneArtCatalog>,
    mut events: EventReader<PlantStageChanged>,
    plants: Query<&Plant>,
) {
    for event in events.read() {
        if let Ok(plant) = plants.get(event.entity) {
            respawn_plant_visual(
                &mut commands,
                event.entity,
                plant.species,
                event.new_stage,
                &assets,
                &art_catalog,
            );
        }
    }
}
