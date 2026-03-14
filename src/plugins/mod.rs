use crate::events::milestones::*;
use crate::resources::*;
use crate::systems;
use bevy::prelude::*;

pub struct TerrariumPlugin;

impl Plugin for TerrariumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeOfDay>()
            .init_resource::<WeatherState>()
            .init_resource::<FeatureToggles>()
            .init_resource::<BehaviorSignals>()
            .init_resource::<DebugSettings>()
            .init_resource::<DebugActions>()
            .init_resource::<DebugTelemetry>()
            .init_resource::<SmokeScript>()
            .add_event::<PlantStageChanged>()
            .add_event::<CritterArrived>()
            .add_event::<CritterDeparted>()
            .add_event::<WeatherChanged>()
            .add_systems(
                Startup,
                (
                    systems::setup::setup_scene,
                    systems::persistence::setup_persistence,
                    #[cfg(feature = "experimental-weather")]
                    systems::rain::setup_rain_assets,
                    #[cfg(feature = "experimental-weather")]
                    systems::fog::setup_fog_assets,
                    #[cfg(feature = "experimental-weather")]
                    systems::wind::setup_wind_assets,
                    systems::adaptive_framerate::setup_framerate_limits,
                    systems::debug_ui::setup_debug_overlay,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    systems::parallax::parallax_system,
                    systems::time_of_day::time_of_day_system,
                    systems::feature_controls::enforce_feature_fallbacks,
                    systems::plant_growth::handle_plant_stage_changes,
                    #[cfg(feature = "experimental-weather")]
                    systems::weather_transition::weather_transition_system,
                    #[cfg(feature = "experimental-weather")]
                    systems::rain::rain_spawn_system,
                    #[cfg(feature = "experimental-weather")]
                    systems::rain::rain_update_system,
                    #[cfg(feature = "experimental-weather")]
                    systems::fog::fog_spawn_system,
                    #[cfg(feature = "experimental-weather")]
                    systems::fog::fog_update_system,
                    #[cfg(feature = "experimental-weather")]
                    systems::wind::wind_spawn_system,
                    #[cfg(feature = "experimental-weather")]
                    systems::wind::wind_update_system,
                    systems::debug_ui::debug_input_system,
                    systems::debug_ui::smoke_script_system,
                    systems::plant_growth::debug_advance_plants_system,
                    systems::critter_spawner::critter_spawner_system,
                    systems::critter_movement::critter_movement_system,
                    systems::persistence::save_state_system,
                    systems::debug_ui::record_milestone_events_system,
                    systems::debug_ui::update_debug_overlay_system,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    systems::plant_growth::plant_growth_system,
                    systems::behavior_tracker::behavior_tracker_system,
                ),
            )
            .add_systems(Last, systems::persistence::save_on_exit_system);
    }
}
