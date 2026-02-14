use bevy::prelude::*;
use crate::resources::{TerrariumPersistentState, BehaviorSignals};
use crate::components::Plant;
use crate::errors::TerrariumError;
use std::path::PathBuf;

#[derive(Resource)]
pub struct PersistenceTimer(pub Timer);

fn state_dir() -> Result<PathBuf, TerrariumError> {
    let dir = dirs::data_dir()
        .ok_or_else(|| TerrariumError::StateWriteFailed {
            path: "unknown".into(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "no data dir"),
        })?
        .join("com.desktopterrarium.app");

    std::fs::create_dir_all(&dir).map_err(|e| TerrariumError::StateWriteFailed {
        path: dir.display().to_string(),
        source: e,
    })?;

    Ok(dir)
}

pub fn setup_persistence(mut commands: Commands) {
    commands.insert_resource(PersistenceTimer(Timer::from_seconds(300.0, TimerMode::Repeating)));
}

pub fn save_state_system(
    mut timer: ResMut<PersistenceTimer>,
    plants: Query<&Plant>,
    behavior: Res<BehaviorSignals>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        // Build state from current game state
        let mut state = TerrariumPersistentState::default();

        for plant in &plants {
            let slot = plant.slot as usize;
            if slot < 3 {
                state.plants[slot].species = plant.species;
                state.plants[slot].stage = plant.stage;
                state.plants[slot].growth_progress = plant.growth_progress;
            }
        }

        state.total_active_secs = behavior.total_active_secs;
        state.longest_focus_streak_secs = behavior.longest_focus_streak_secs;

        // Save to disk
        if let Err(e) = save_state(&state) {
            warn!("Failed to save state: {}", e);
        }
    }
}

fn save_state(state: &TerrariumPersistentState) -> Result<(), TerrariumError> {
    let dir = state_dir()?;
    let temp_path = dir.join("state.tmp.json");
    let final_path = dir.join("state.json");

    let json = serde_json::to_string_pretty(state)
        .map_err(|e| TerrariumError::StateWriteFailed {
            path: temp_path.display().to_string(),
            source: std::io::Error::new(std::io::ErrorKind::Other, e),
        })?;

    std::fs::write(&temp_path, json).map_err(|e| TerrariumError::StateWriteFailed {
        path: temp_path.display().to_string(),
        source: e,
    })?;

    std::fs::rename(&temp_path, &final_path).map_err(|e| TerrariumError::StateWriteFailed {
        path: final_path.display().to_string(),
        source: e,
    })?;

    Ok(())
}
