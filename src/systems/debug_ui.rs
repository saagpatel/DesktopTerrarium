use crate::components::{Critter, CritterSpecies, Plant};
use crate::events::{CritterArrived, CritterDeparted, PlantStageChanged, WeatherChanged};
use crate::resources::{
    ActivityMode, BehaviorSignals, DebugActions, DebugSettings, DebugTelemetry, FeatureToggles,
    SmokeScript, TimeOfDay, WeatherState, WeatherType,
};
use crate::systems::persistence::PersistenceTimer;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

const DEBUG_FONT_PATH: &str = "fonts/SFNSMono.ttf";

#[derive(Component)]
pub struct DebugOverlayRoot;

#[derive(Component)]
pub struct DebugOverlayText;

#[derive(SystemParam)]
pub struct DebugOverlaySnapshot<'w, 's> {
    behavior: Res<'w, BehaviorSignals>,
    critters: Query<'w, 's, &'static Critter>,
    debug: Res<'w, DebugSettings>,
    persistence_timer: Res<'w, PersistenceTimer>,
    plants: Query<'w, 's, &'static Plant>,
    telemetry: Res<'w, DebugTelemetry>,
    time_of_day: Res<'w, TimeOfDay>,
    toggles: Res<'w, FeatureToggles>,
    weather: Res<'w, WeatherState>,
}

pub fn setup_debug_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                max_width: Val::Px(420.0),
                padding: UiRect::all(Val::Px(12.0)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.08, 0.10, 0.92)),
            ZIndex(100),
            DebugOverlayRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Desktop Terrarium debug overlay"),
                TextFont {
                    font: asset_server.load(DEBUG_FONT_PATH),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.93, 0.95, 0.98)),
                DebugOverlayText,
            ));
        });
}

pub fn debug_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    toggles: Res<FeatureToggles>,
    mut actions: ResMut<DebugActions>,
    mut debug: ResMut<DebugSettings>,
    mut telemetry: ResMut<DebugTelemetry>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        debug.show_overlay = !debug.show_overlay;
        telemetry.push_event(if debug.show_overlay {
            "Debug overlay enabled"
        } else {
            "Debug overlay hidden"
        });
    }

    if keyboard.just_pressed(KeyCode::F2) {
        debug.force_time_phase = match debug.force_time_phase {
            None => Some(0),
            Some(0) => Some(1),
            Some(1) => Some(2),
            Some(2) => Some(3),
            Some(3) => None,
            _ => None,
        };
        telemetry.push_event(match debug.force_time_phase {
            Some(phase) => format!("Forced time phase -> {phase}"),
            None => "Time phase override cleared".to_string(),
        });
    }

    if keyboard.just_pressed(KeyCode::F3) {
        debug.activity_mode = debug.activity_mode.next();
        telemetry.push_event(format!("Activity mode -> {}", debug.activity_mode.label()));
    }

    if keyboard.just_pressed(KeyCode::F4) {
        if toggles.weather_transitions_enabled() {
            debug.force_weather = match debug.force_weather {
                None => Some(WeatherType::Clear),
                Some(WeatherType::Clear) => Some(WeatherType::Fog),
                Some(WeatherType::Fog) => Some(WeatherType::Rain),
                Some(WeatherType::Rain) => Some(WeatherType::Wind),
                Some(WeatherType::Wind) => None,
            };
            telemetry.push_event(match debug.force_weather {
                Some(weather) => format!("Forced weather -> {:?}", weather),
                None => "Weather override cleared".to_string(),
            });
        } else {
            telemetry.push_event("Weather override unavailable in forced-clear mode");
        }
    }

    if keyboard.just_pressed(KeyCode::F5) {
        debug.cycle_time_scale();
        telemetry.push_event(format!("Time scale -> {:.0}x", debug.time_scale));
    }

    if keyboard.just_pressed(KeyCode::F6) {
        debug.cycle_growth_rate_multiplier();
        telemetry.push_event(format!(
            "Growth multiplier -> {:.0}x",
            debug.growth_rate_multiplier
        ));
    }

    if keyboard.just_pressed(KeyCode::F7) {
        actions.advance_plants = true;
        telemetry.push_event("Queued plant stage advance");
    }

    if keyboard.just_pressed(KeyCode::F8) {
        actions.spawn_critter = Some(CritterSpecies::Beetle);
        telemetry.push_event("Queued beetle spawn");
    }

    if keyboard.just_pressed(KeyCode::F9) {
        actions.spawn_critter = Some(CritterSpecies::Butterfly);
        telemetry.push_event("Queued butterfly spawn");
    }

    if keyboard.just_pressed(KeyCode::F10) {
        actions.save_state = true;
        telemetry.push_event("Queued manual save");
    }
}

pub fn smoke_script_system(
    time: Res<Time>,
    mut actions: ResMut<DebugActions>,
    mut debug: ResMut<DebugSettings>,
    mut smoke_script: ResMut<SmokeScript>,
    mut telemetry: ResMut<DebugTelemetry>,
) {
    if !smoke_script.enabled {
        return;
    }

    smoke_script.timer.tick(time.delta());
    if !smoke_script.timer.just_finished() {
        return;
    }

    match smoke_script.step {
        0 => {
            debug.show_overlay = true;
            debug.activity_mode = ActivityMode::ForceActive;
            telemetry.push_event("Smoke script: overlay on, activity forced active");
        }
        1 => {
            debug.force_time_phase = Some(0);
            telemetry.push_event("Smoke script: morning phase forced");
        }
        2 => {
            debug.force_weather = Some(WeatherType::Rain);
            telemetry.push_event("Smoke script: rain forced");
        }
        3 => {
            debug.growth_rate_multiplier = 250.0;
            telemetry.push_event("Smoke script: growth multiplier set to 250x");
        }
        4 => {
            actions.advance_plants = true;
            telemetry.push_event("Smoke script: plant stage advance queued");
        }
        5 => {
            actions.spawn_critter = Some(CritterSpecies::Beetle);
            telemetry.push_event("Smoke script: beetle spawn queued");
        }
        6 => {
            actions.spawn_critter = Some(CritterSpecies::Butterfly);
            telemetry.push_event("Smoke script: butterfly spawn queued");
        }
        7 => {
            actions.save_state = true;
            actions.exit_after_save = true;
            telemetry.push_event("Smoke script: save queued and exit requested");
            smoke_script.enabled = false;
        }
        _ => {
            smoke_script.enabled = false;
        }
    }

    smoke_script.step += 1;
}

pub fn record_milestone_events_system(
    mut critter_arrivals: EventReader<CritterArrived>,
    mut critter_departures: EventReader<CritterDeparted>,
    mut plant_changes: EventReader<PlantStageChanged>,
    mut weather_changes: EventReader<WeatherChanged>,
    mut telemetry: ResMut<DebugTelemetry>,
) {
    for event in plant_changes.read() {
        telemetry.push_event(format!(
            "Plant {:?} stage {} -> {}",
            event.species, event.old_stage, event.new_stage
        ));
    }

    for event in critter_arrivals.read() {
        telemetry.push_event(format!("{:?} arrived", event.species));
    }

    for event in critter_departures.read() {
        telemetry.push_event(format!("{:?} departed", event.species));
    }

    for event in weather_changes.read() {
        telemetry.push_event(format!("Weather {:?} -> {:?}", event.from, event.to));
    }
}

pub fn update_debug_overlay_system(
    mut overlay_root: Query<&mut Node, With<DebugOverlayRoot>>,
    mut overlay_text: Query<&mut Text, With<DebugOverlayText>>,
    snapshot: DebugOverlaySnapshot,
) {
    let Ok(mut root) = overlay_root.get_single_mut() else {
        return;
    };

    root.display = if snapshot.debug.show_overlay {
        Display::Flex
    } else {
        Display::None
    };
    if !snapshot.debug.show_overlay {
        return;
    }

    let Ok(mut text) = overlay_text.get_single_mut() else {
        return;
    };

    let mut plant_rows = snapshot
        .plants
        .iter()
        .map(|plant| {
            format!(
                "slot {} {:?}: stage {} ({:.0}%)",
                plant.slot,
                plant.species,
                plant.stage,
                plant.growth_progress * 100.0
            )
        })
        .collect::<Vec<_>>();
    plant_rows.sort();

    let mut beetles = 0usize;
    let mut butterflies = 0usize;
    for critter in &snapshot.critters {
        match critter.species {
            CritterSpecies::Beetle => beetles += 1,
            CritterSpecies::Butterfly => butterflies += 1,
        }
    }

    let forced_time = snapshot
        .debug
        .force_time_phase
        .map(|phase| phase.to_string())
        .unwrap_or_else(|| "none".to_string());
    let forced_weather = snapshot
        .debug
        .force_weather
        .map(|weather| format!("{weather:?}"))
        .unwrap_or_else(|| "none".to_string());
    let state_path = snapshot
        .telemetry
        .state_file_path
        .as_deref()
        .unwrap_or("unknown");
    let autosave_remaining = snapshot.persistence_timer.0.remaining_secs();
    let recent_events = if snapshot.telemetry.recent_events.is_empty() {
        "  - none yet".to_string()
    } else {
        snapshot
            .telemetry
            .recent_events
            .iter()
            .map(|event| format!("  - {event}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    **text = format!(
        "Desktop Terrarium Debug\n\
controls: F2 phase | F3 activity | F4 weather | F5 time | F6 growth | F7 stage+1 | F8 beetle | F9 butterfly | F10 save\n\
\n\
time: {} phase={} progress={:.0}% forced={}\n\
weather: mode={:?} current={:?} target={:?} transition={:.0}% forced={}\n\
activity: mode={} active={} idle={:.1}s streak={:.1}s total={:.1}s\n\
critters: beetles={} butterflies={}\n\
plants:\n{}\n\
\n\
persistence: next autosave in {:.1}s | last save={} \n\
state file: {}\n\
\n\
recent events:\n{}",
        snapshot.time_of_day.phase_name(),
        snapshot.time_of_day.phase,
        snapshot.time_of_day.progress * 100.0,
        forced_time,
        snapshot.toggles.weather_mode,
        snapshot.weather.current,
        snapshot.weather.target,
        snapshot.weather.transition_progress * 100.0,
        forced_weather,
        snapshot.debug.activity_mode.label(),
        snapshot.behavior.is_active,
        snapshot.behavior.system_idle_secs,
        snapshot.behavior.current_focus_streak_secs,
        snapshot.behavior.total_active_secs,
        beetles,
        butterflies,
        plant_rows.join("\n"),
        autosave_remaining,
        snapshot.telemetry.last_save_status,
        state_path,
        recent_events,
    );
}
