use crate::components::Plant;
use crate::errors::TerrariumError;
use crate::resources::{BehaviorSignals, TerrariumPersistentState, TimeOfDay, WeatherState};
use bevy::prelude::*;
use std::path::{Path, PathBuf};

const STATE_DIR_NAME: &str = "com.desktopterrarium.app";
const STATE_FILE_NAME: &str = "state.json";
const TEMP_STATE_FILE_NAME: &str = "state.tmp.json";
const SUPPORTED_STATE_VERSION: u32 = 1;

#[derive(Resource)]
pub struct PersistenceTimer(pub Timer);

fn state_dir() -> Result<PathBuf, TerrariumError> {
    let dir = dirs::data_dir()
        .ok_or_else(|| TerrariumError::StateWriteFailed {
            path: "unknown".into(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "no data dir"),
        })?
        .join(STATE_DIR_NAME);

    std::fs::create_dir_all(&dir).map_err(|e| TerrariumError::StateWriteFailed {
        path: dir.display().to_string(),
        source: e,
    })?;

    Ok(dir)
}

fn state_paths(dir: &Path) -> (PathBuf, PathBuf) {
    (dir.join(TEMP_STATE_FILE_NAME), dir.join(STATE_FILE_NAME))
}

pub fn setup_persistence(
    mut commands: Commands,
    mut plants: Query<&mut Plant>,
    mut behavior: ResMut<BehaviorSignals>,
    mut weather: ResMut<WeatherState>,
    mut time_of_day: ResMut<TimeOfDay>,
) {
    commands.insert_resource(PersistenceTimer(Timer::from_seconds(
        300.0,
        TimerMode::Repeating,
    )));

    let state = match load_state() {
        Ok(Some(state)) => state,
        Ok(None) => TerrariumPersistentState::default(),
        Err(e) => {
            warn!("Failed to load persisted state, using defaults: {}", e);
            TerrariumPersistentState::default()
        }
    };

    apply_loaded_state(
        &state,
        &mut plants,
        &mut behavior,
        &mut weather,
        &mut time_of_day,
    );

    commands.insert_resource(state);
}

fn apply_loaded_state(
    state: &TerrariumPersistentState,
    plants: &mut Query<&mut Plant>,
    behavior: &mut BehaviorSignals,
    weather: &mut WeatherState,
    time_of_day: &mut TimeOfDay,
) {
    for mut plant in plants.iter_mut() {
        let slot = plant.slot as usize;
        if slot < state.plants.len() {
            let saved = &state.plants[slot];
            plant.species = saved.species;
            plant.stage = saved.stage.min(4);
            plant.growth_progress = saved.growth_progress.clamp(0.0, 1.0);
        }
    }

    behavior.total_active_secs = state.total_active_secs.max(0.0);
    behavior.longest_focus_streak_secs = state.longest_focus_streak_secs.max(0.0);
    behavior.current_focus_streak_secs = 0.0;

    weather.current = state.weather;
    weather.target = state.weather;
    weather.transition_progress = 1.0;
    weather.phase_elapsed = 0.0;

    time_of_day.phase = state.time_of_day_phase % 4;
    time_of_day.progress = 0.0;
}

pub fn save_state_system(
    mut timer: ResMut<PersistenceTimer>,
    mut persistent_state: ResMut<TerrariumPersistentState>,
    plants: Query<&Plant>,
    behavior: Res<BehaviorSignals>,
    weather: Res<WeatherState>,
    time_of_day: Res<TimeOfDay>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        for plant in &plants {
            let slot = plant.slot as usize;
            if slot < persistent_state.plants.len() {
                persistent_state.plants[slot].species = plant.species;
                persistent_state.plants[slot].stage = plant.stage.min(4);
                persistent_state.plants[slot].growth_progress =
                    plant.growth_progress.clamp(0.0, 1.0);
            }
        }

        persistent_state.version = SUPPORTED_STATE_VERSION;
        if persistent_state.created_at.is_empty() {
            persistent_state.created_at = chrono::Utc::now().to_rfc3339();
        }

        persistent_state.total_active_secs = behavior.total_active_secs.max(0.0);
        persistent_state.longest_focus_streak_secs = behavior.longest_focus_streak_secs.max(0.0);
        persistent_state.weather = weather.current;
        persistent_state.time_of_day_phase = time_of_day.phase % 4;

        if let Err(e) = save_state(&persistent_state) {
            warn!("Failed to save state: {}", e);
        }
    }
}

fn load_state() -> Result<Option<TerrariumPersistentState>, TerrariumError> {
    let dir = state_dir()?;
    load_state_from_dir(&dir)
}

fn load_state_from_dir(dir: &Path) -> Result<Option<TerrariumPersistentState>, TerrariumError> {
    let (_, final_path) = state_paths(dir);

    if !final_path.exists() {
        return Ok(None);
    }

    let json =
        std::fs::read_to_string(&final_path).map_err(|e| TerrariumError::StateReadFailed {
            path: final_path.display().to_string(),
            source: e,
        })?;

    let mut state: TerrariumPersistentState = serde_json::from_str(&json)
        .map_err(|e| TerrariumError::StateDeserializeFailed { source: e })?;

    if state.version > SUPPORTED_STATE_VERSION {
        return Err(TerrariumError::UnsupportedStateVersion {
            found: state.version,
            supported: SUPPORTED_STATE_VERSION,
        });
    }

    if state.version == 0 {
        state.version = SUPPORTED_STATE_VERSION;
    }

    if state.created_at.is_empty() {
        state.created_at = chrono::Utc::now().to_rfc3339();
    }

    state.time_of_day_phase %= 4;
    for plant in &mut state.plants {
        plant.stage = plant.stage.min(4);
        plant.growth_progress = plant.growth_progress.clamp(0.0, 1.0);
    }
    state.total_active_secs = state.total_active_secs.max(0.0);
    state.longest_focus_streak_secs = state.longest_focus_streak_secs.max(0.0);

    Ok(Some(state))
}

fn save_state(state: &TerrariumPersistentState) -> Result<(), TerrariumError> {
    let dir = state_dir()?;
    save_state_to_dir(&dir, state)
}

fn save_state_to_dir(dir: &Path, state: &TerrariumPersistentState) -> Result<(), TerrariumError> {
    std::fs::create_dir_all(dir).map_err(|e| TerrariumError::StateWriteFailed {
        path: dir.display().to_string(),
        source: e,
    })?;

    let (temp_path, final_path) = state_paths(dir);

    let json =
        serde_json::to_string_pretty(state).map_err(|e| TerrariumError::StateWriteFailed {
            path: temp_path.display().to_string(),
            source: std::io::Error::other(e),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::PlantSpecies;
    use crate::resources::WeatherType;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn approx_eq(lhs: f64, rhs: f64) {
        assert!((lhs - rhs).abs() < 1e-9, "left={lhs}, right={rhs}");
    }

    fn test_dir(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "desktop_terrarium_{name}_{}_{}",
            std::process::id(),
            stamp
        ))
    }

    fn cleanup(path: &Path) {
        let _ = std::fs::remove_dir_all(path);
    }

    #[test]
    fn save_and_load_round_trip_preserves_metadata() {
        let dir = test_dir("round_trip");

        let mut state = TerrariumPersistentState {
            created_at: "2026-01-01T00:00:00Z".to_string(),
            total_active_secs: 123.5,
            longest_focus_streak_secs: 99.25,
            weather: WeatherType::Rain,
            time_of_day_phase: 3,
            ..Default::default()
        };
        state.plants[1].species = PlantSpecies::Succulent;
        state.plants[1].stage = 4;
        state.plants[1].growth_progress = 1.0;

        save_state_to_dir(&dir, &state).expect("save should succeed");
        let loaded = load_state_from_dir(&dir)
            .expect("load should succeed")
            .expect("state should exist");

        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.created_at, "2026-01-01T00:00:00Z");
        approx_eq(loaded.total_active_secs, 123.5);
        approx_eq(loaded.longest_focus_streak_secs, 99.25);
        assert_eq!(loaded.weather, WeatherType::Rain);
        assert_eq!(loaded.time_of_day_phase, 3);
        assert_eq!(loaded.plants[1].species, PlantSpecies::Succulent);
        assert_eq!(loaded.plants[1].stage, 4);
        assert!((loaded.plants[1].growth_progress - 1.0).abs() < 1e-9);

        cleanup(&dir);
    }

    #[test]
    fn unsupported_state_version_is_rejected() {
        let dir = test_dir("unsupported");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let (_, final_path) = state_paths(&dir);
        let payload = r#"{
  "version": 2,
  "created_at": "2026-01-01T00:00:00Z",
  "plants": [
    {"species": "Fern", "stage": 0, "growth_progress": 0.0},
    {"species": "Moss", "stage": 0, "growth_progress": 0.0},
    {"species": "Succulent", "stage": 0, "growth_progress": 0.0}
  ],
  "total_active_secs": 0.0,
  "longest_focus_streak_secs": 0.0,
  "weather": "Clear",
  "time_of_day_phase": 1
}"#;
        std::fs::write(&final_path, payload).expect("write state file");

        match load_state_from_dir(&dir) {
            Err(TerrariumError::UnsupportedStateVersion { found, supported }) => {
                assert_eq!(found, 2);
                assert_eq!(supported, 1);
            }
            Ok(_) => panic!("expected error for unsupported version"),
            Err(_) => panic!("expected UnsupportedStateVersion"),
        }

        cleanup(&dir);
    }

    #[test]
    fn missing_state_file_returns_none() {
        let dir = test_dir("missing");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let loaded = load_state_from_dir(&dir).expect("load should not fail");
        assert!(loaded.is_none());

        cleanup(&dir);
    }
}
