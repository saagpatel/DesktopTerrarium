use crate::events::milestones::*;
use crate::resources::*;
use crate::systems;
use bevy::prelude::*;

pub struct TerrariumPlugin;

impl Plugin for TerrariumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeOfDay>()
            .init_resource::<WeatherState>()
            .init_resource::<BehaviorSignals>()
            .init_resource::<DebugSettings>()
            .add_message::<PlantStageChanged>()
            .add_message::<CritterArrived>()
            .add_message::<CritterDeparted>()
            .add_message::<WeatherChanged>()
            .add_systems(
                Startup,
                (
                    systems::setup::setup_scene,
                    systems::persistence::setup_persistence,
                    systems::rain::setup_rain_assets,
                    systems::fog::setup_fog_assets,
                    systems::wind::setup_wind_assets,
                    systems::adaptive_framerate::setup_framerate_limits,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    systems::parallax::parallax_system,
                    systems::time_of_day::time_of_day_system,
                    systems::plant_growth::handle_plant_stage_changes,
                    systems::weather_transition::weather_transition_system,
                    systems::rain::rain_spawn_system,
                    systems::rain::rain_update_system,
                    systems::fog::fog_spawn_system,
                    systems::fog::fog_update_system,
                    systems::wind::wind_spawn_system,
                    systems::wind::wind_update_system,
                    systems::critter_spawner::critter_spawner_system,
                    systems::critter_movement::critter_movement_system,
                    systems::persistence::save_state_system,
                    systems::debug_ui::debug_input_system,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    systems::plant_growth::plant_growth_system,
                    systems::behavior_tracker::behavior_tracker_system,
                ),
            );
    }
}
