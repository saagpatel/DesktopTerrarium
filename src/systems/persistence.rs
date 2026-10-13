use crate::components::Plant;
use crate::errors::TerrariumError;
use crate::resources::{BehaviorSignals, TerrariumPersistentState, TimeOfDay, WeatherState};
use bevy::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const STATE_DIR_NAME: &str = "com.desktopterrarium.app";
const STATE_FILE_NAME: &str = "state.json";
const SUPPORTED_STATE_VERSION: u32 = 1;
static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Resource)]
pub struct PersistenceControl {
    timer: Timer,
    save_allowed: bool,
}

enum LoadOutcome {
    Missing,
    Loaded(TerrariumPersistentState),
    Rejected(TerrariumError),
}

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

fn state_path(dir: &Path) -> PathBuf {
    dir.join(STATE_FILE_NAME)
}

pub fn setup_persistence(
    mut commands: Commands,
    mut plants: Query<(&mut Plant, &mut Sprite)>,
    mut behavior: ResMut<BehaviorSignals>,
    mut weather: ResMut<WeatherState>,
    mut time_of_day: ResMut<TimeOfDay>,
    asset_server: Res<AssetServer>,
) {
    let (state, save_allowed) = match load_state() {
        Ok(LoadOutcome::Loaded(state)) => (state, true),
        Ok(LoadOutcome::Missing) => (TerrariumPersistentState::default(), true),
        Ok(LoadOutcome::Rejected(error)) => {
            warn!("Rejected persisted state; preserving it and using defaults: {error}");
            (TerrariumPersistentState::default(), false)
        }
        Err(error) => {
            warn!("Failed to inspect persisted state; saving is disabled: {error}");
            (TerrariumPersistentState::default(), false)
        }
    };

    commands.insert_resource(PersistenceControl {
        timer: Timer::from_seconds(300.0, TimerMode::Repeating),
        save_allowed,
    });

    apply_loaded_state(
        &state,
        &mut plants,
        &mut behavior,
        &mut weather,
        &mut time_of_day,
        &asset_server,
    );

    commands.insert_resource(state);
}

fn apply_loaded_state(
    state: &TerrariumPersistentState,
    plants: &mut Query<(&mut Plant, &mut Sprite)>,
    behavior: &mut BehaviorSignals,
    weather: &mut WeatherState,
    time_of_day: &mut TimeOfDay,
    asset_server: &AssetServer,
) {
    for (mut plant, mut sprite) in plants.iter_mut() {
        let slot = plant.slot as usize;
        if slot < state.plants.len() {
            let saved = &state.plants[slot];
            plant.species = saved.species;
            plant.stage = saved.stage.min(4);
            plant.growth_progress = saved.growth_progress.clamp(0.0, 1.0);
            sprite.image = asset_server.load(plant_sprite_path(plant.species, plant.stage));
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

fn plant_sprite_path(species: crate::components::PlantSpecies, stage: u8) -> String {
    format!("plants/{}_stage{}.png", species.asset_name(), stage.min(4))
}

pub fn save_state_system(
    mut control: ResMut<PersistenceControl>,
    mut persistent_state: ResMut<TerrariumPersistentState>,
    plants: Query<&Plant>,
    behavior: Res<BehaviorSignals>,
    weather: Res<WeatherState>,
    time_of_day: Res<TimeOfDay>,
    time: Res<Time>,
) {
    control.timer.tick(time.delta());

    if control.timer.just_finished() && control.save_allowed {
        update_persistent_state(
            &mut persistent_state,
            &plants,
            &behavior,
            &weather,
            &time_of_day,
        );
        if let Err(e) = autosave_state(&persistent_state, control.save_allowed) {
            warn!("Failed to save state: {}", e);
        }
    }
}

fn autosave_state(state: &TerrariumPersistentState, allowed: bool) -> Result<(), TerrariumError> {
    if allowed {
        save_state(state)
    } else {
        Ok(())
    }
}

#[cfg(test)]
fn autosave_state_to_dir(
    dir: &Path,
    state: &TerrariumPersistentState,
    allowed: bool,
) -> Result<(), TerrariumError> {
    if allowed {
        save_state_to_dir(dir, state)
    } else {
        Ok(())
    }
}

pub fn save_state_on_exit_system(
    mut exits: MessageReader<AppExit>,
    control: Res<PersistenceControl>,
    mut persistent_state: ResMut<TerrariumPersistentState>,
    plants: Query<&Plant>,
    behavior: Res<BehaviorSignals>,
    weather: Res<WeatherState>,
    time_of_day: Res<TimeOfDay>,
) {
    if exits.read().next().is_some() && control.save_allowed {
        update_persistent_state(
            &mut persistent_state,
            &plants,
            &behavior,
            &weather,
            &time_of_day,
        );
        if let Err(error) = save_state_on_exit(&persistent_state, true) {
            warn!("Failed to save state on exit: {error}");
        }
    }
}

fn save_state_on_exit(
    state: &TerrariumPersistentState,
    exit_requested: bool,
) -> Result<(), TerrariumError> {
    if exit_requested {
        save_state(state)
    } else {
        Ok(())
    }
}

#[cfg(test)]
fn save_state_on_exit_to_dir(
    dir: &Path,
    state: &TerrariumPersistentState,
    exit_requested: bool,
) -> Result<(), TerrariumError> {
    if exit_requested {
        save_state_to_dir(dir, state)
    } else {
        Ok(())
    }
}

fn update_persistent_state(
    persistent_state: &mut TerrariumPersistentState,
    plants: &Query<&Plant>,
    behavior: &BehaviorSignals,
    weather: &WeatherState,
    time_of_day: &TimeOfDay,
) {
    for plant in plants {
        let slot = plant.slot as usize;
        if slot < persistent_state.plants.len() {
            persistent_state.plants[slot].species = plant.species;
            persistent_state.plants[slot].stage = plant.stage.min(4);
            persistent_state.plants[slot].growth_progress = plant.growth_progress.clamp(0.0, 1.0);
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
}

fn load_state() -> Result<LoadOutcome, TerrariumError> {
    let dir = state_dir()?;
    load_state_from_dir(&dir)
}

fn load_state_from_dir(dir: &Path) -> Result<LoadOutcome, TerrariumError> {
    let final_path = state_path(dir);

    if !final_path.exists() {
        return Ok(LoadOutcome::Missing);
    }

    let loaded = (|| {
        let json = std::fs::read_to_string(&final_path).map_err(|source| {
            TerrariumError::StateReadFailed {
                path: final_path.display().to_string(),
                source,
            }
        })?;
        let mut state: TerrariumPersistentState = serde_json::from_str(&json)
            .map_err(|source| TerrariumError::StateDeserializeFailed { source })?;
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
        Ok(state)
    })();

    match loaded {
        Ok(state) => Ok(LoadOutcome::Loaded(state)),
        Err(error) => {
            let rejected_path = final_path.with_extension("json.rejected");
            if let Err(copy_error) = std::fs::copy(&final_path, &rejected_path) {
                warn!(
                    "Could not copy rejected state to {}: {}",
                    rejected_path.display(),
                    copy_error
                );
            }
            Ok(LoadOutcome::Rejected(error))
        }
    }
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

    let final_path = state_path(dir);
    let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp_path = dir.join(format!(
        "state.{}.{}.tmp.json",
        std::process::id(),
        sequence
    ));

    let json =
        serde_json::to_string_pretty(state).map_err(|e| TerrariumError::StateWriteFailed {
            path: temp_path.display().to_string(),
            source: std::io::Error::other(e),
        })?;

    let write_result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
            .map_err(|source| TerrariumError::StateWriteFailed {
                path: temp_path.display().to_string(),
                source,
            })?;
        file.write_all(json.as_bytes())
            .map_err(|source| TerrariumError::StateWriteFailed {
                path: temp_path.display().to_string(),
                source,
            })?;
        file.sync_all()
            .map_err(|source| TerrariumError::StateWriteFailed {
                path: temp_path.display().to_string(),
                source,
            })?;
        std::fs::rename(&temp_path, &final_path).map_err(|source| {
            TerrariumError::StateWriteFailed {
                path: final_path.display().to_string(),
                source,
            }
        })?;
        Ok(())
    })();
    if write_result.is_err() {
        let _ = std::fs::remove_file(&temp_path);
    }
    write_result?;

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
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/test-temp")
            .join(format!(
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
        let loaded = match load_state_from_dir(&dir).expect("load should succeed") {
            LoadOutcome::Loaded(state) => state,
            _ => panic!("state should be loaded"),
        };

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

        let final_path = state_path(&dir);
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

        match load_state_from_dir(&dir).expect("load outcome") {
            LoadOutcome::Rejected(TerrariumError::UnsupportedStateVersion { found, supported }) => {
                assert_eq!(found, 2);
                assert_eq!(supported, 1);
            }
            _ => panic!("expected UnsupportedStateVersion"),
        }

        cleanup(&dir);
    }

    #[test]
    fn missing_state_file_returns_none() {
        let dir = test_dir("missing");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let loaded = load_state_from_dir(&dir).expect("load should not fail");
        assert!(matches!(loaded, LoadOutcome::Missing));

        cleanup(&dir);
    }

    #[test]
    fn rejected_saves_are_preserved_through_autosave() {
        let corrupt = b"{not valid json";
        let future_state = TerrariumPersistentState {
            version: 2,
            ..Default::default()
        };
        let future = serde_json::to_vec(&future_state).expect("serialize future state");
        for (name, payload) in [
            ("corrupt", corrupt.as_slice()),
            ("future", future.as_slice()),
        ] {
            let dir = test_dir(name);
            std::fs::create_dir_all(&dir).expect("create test dir");
            let final_path = state_path(&dir);
            std::fs::write(&final_path, payload).expect("write source save");

            let permission = match load_state_from_dir(&dir).expect("load outcome") {
                LoadOutcome::Rejected(_) => false,
                _ => panic!("save should be rejected"),
            };
            autosave_state_to_dir(&dir, &TerrariumPersistentState::default(), permission)
                .expect("blocked autosave should be harmless");

            assert_eq!(std::fs::read(&final_path).expect("read source"), payload);
            assert_eq!(
                std::fs::read(final_path.with_extension("json.rejected"))
                    .expect("read rejected copy"),
                payload
            );
            cleanup(&dir);
        }
    }

    #[test]
    fn save_writes_use_fresh_temporary_files() {
        let dir = test_dir("unique_temps");
        let state = TerrariumPersistentState::default();
        save_state_to_dir(&dir, &state).expect("first save");
        save_state_to_dir(&dir, &state).expect("second save");
        assert!(state_path(&dir).exists());
        assert_eq!(
            std::fs::read_dir(&dir).expect("read dir").count(),
            1,
            "temporary files should be renamed away"
        );
        cleanup(&dir);
    }

    #[test]
    fn failed_save_cleans_up_its_temporary_file() {
        let dir = test_dir("failed_save_cleanup");
        std::fs::create_dir_all(state_path(&dir)).expect("make final path a directory");
        assert!(save_state_to_dir(&dir, &TerrariumPersistentState::default()).is_err());
        assert_eq!(
            std::fs::read_dir(&dir).expect("read dir").count(),
            1,
            "failed writes should leave no temporary file"
        );
        cleanup(&dir);
    }

    #[test]
    fn exit_flush_writes_to_the_supplied_directory() {
        let dir = test_dir("exit_flush");
        let state = TerrariumPersistentState::default();
        save_state_on_exit_to_dir(&dir, &state, true).expect("exit save");
        assert!(state_path(&dir).exists());
        assert!(save_state_on_exit_to_dir(&dir, &state, false).is_ok());
        cleanup(&dir);
    }

    #[test]
    fn every_restored_species_stage_maps_to_its_sprite() {
        for species in [
            PlantSpecies::Fern,
            PlantSpecies::Moss,
            PlantSpecies::Succulent,
        ] {
            for stage in 0..=4 {
                assert_eq!(
                    plant_sprite_path(species, stage),
                    format!("plants/{}_stage{}.png", species.asset_name(), stage)
                );
            }
        }
    }
}
