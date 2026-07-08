use crate::components::Plant;
use crate::events::PlantStageChanged;
use crate::resources::BehaviorSignals;
use bevy::prelude::*;

const GROWTH_RATE_PER_SEC: f32 = 0.001; // ~17 minutes per stage

pub fn plant_growth_system(
    mut plants: Query<(Entity, &mut Plant)>,
    behavior: Res<BehaviorSignals>,
    mut events: MessageWriter<PlantStageChanged>,
    time: Res<Time>,
) {
    let growth_delta = if behavior.is_active {
        GROWTH_RATE_PER_SEC * time.delta_secs()
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

            events.write(PlantStageChanged {
                entity,
                species: plant.species,
                old_stage,
                new_stage: plant.stage,
            });
        }
    }
}

pub fn handle_plant_stage_changes(
    mut events: MessageReader<PlantStageChanged>,
    mut plants: Query<(&Plant, &mut Sprite)>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        if let Ok((plant, mut sprite)) = plants.get_mut(event.entity) {
            let path = format!(
                "plants/{}_stage{}.png",
                plant.species.asset_name(),
                event.new_stage
            );
            sprite.image = asset_server.load(&path);
        }
    }
}
