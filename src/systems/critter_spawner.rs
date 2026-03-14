use crate::components::{Critter, CritterSpecies};
use crate::events::CritterArrived;
use crate::resources::{BehaviorSignals, DebugActions, DebugTelemetry};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use rand::Rng;

#[derive(SystemParam)]
pub struct CritterSpawnContext<'w, 's> {
    commands: Commands<'w, 's>,
    time: Res<'w, Time>,
    asset_server: Res<'w, AssetServer>,
    actions: ResMut<'w, DebugActions>,
    telemetry: ResMut<'w, DebugTelemetry>,
    events: EventWriter<'w, CritterArrived>,
    spawn_timer: Local<'s, f32>,
}

pub fn critter_spawner_system(
    behavior: Res<BehaviorSignals>,
    existing_critters: Query<&Critter>,
    mut ctx: CritterSpawnContext,
) {
    *ctx.spawn_timer += ctx.time.delta_secs();

    if let Some(species) = ctx.actions.spawn_critter.take() {
        if existing_critters
            .iter()
            .any(|critter| critter.species == species)
        {
            ctx.telemetry.push_event(format!(
                "Skipped {:?} spawn: one is already in the terrarium",
                species
            ));
        } else {
            spawn_critter(
                &mut ctx.commands,
                &ctx.asset_server,
                species,
                manual_smoke_path(species),
                critter_speed(species) * 4.0,
            );
            ctx.events.send(CritterArrived { species });
        }
    }

    // Butterfly: spawns after 30 minutes of focus
    if behavior.current_focus_streak_secs >= 1800.0 {
        let has_butterfly = existing_critters
            .iter()
            .any(|c| c.species == CritterSpecies::Butterfly);
        if !has_butterfly && *ctx.spawn_timer > 5.0 {
            let mut rng = rand::thread_rng();
            spawn_critter(
                &mut ctx.commands,
                &ctx.asset_server,
                CritterSpecies::Butterfly,
                random_critter_path(&mut rng),
                critter_speed(CritterSpecies::Butterfly),
            );

            ctx.events.send(CritterArrived {
                species: CritterSpecies::Butterfly,
            });
            *ctx.spawn_timer = 0.0;
        }
    }

    // Beetle: 5% chance per minute when active
    if behavior.is_active && *ctx.spawn_timer > 60.0 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.05 {
            let has_beetle = existing_critters
                .iter()
                .any(|c| c.species == CritterSpecies::Beetle);
            if !has_beetle {
                spawn_critter(
                    &mut ctx.commands,
                    &ctx.asset_server,
                    CritterSpecies::Beetle,
                    random_critter_path(&mut rng),
                    critter_speed(CritterSpecies::Beetle),
                );

                ctx.events.send(CritterArrived {
                    species: CritterSpecies::Beetle,
                });
            }
        }
        *ctx.spawn_timer = 0.0;
    }
}

fn spawn_critter(
    commands: &mut Commands,
    asset_server: &AssetServer,
    species: CritterSpecies,
    path: [Vec2; 4],
    speed: f32,
) {
    let sprite_path = match species {
        CritterSpecies::Butterfly => "critters/butterfly.png",
        CritterSpecies::Beetle => "critters/beetle.png",
    };

    commands.spawn((
        Sprite {
            image: asset_server.load(sprite_path),
            ..default()
        },
        Transform::from_xyz(path[0].x, path[0].y, 40.0),
        Critter {
            species,
            path_progress: 0.0,
            speed,
            path,
        },
    ));
}

fn critter_speed(species: CritterSpecies) -> f32 {
    match species {
        CritterSpecies::Butterfly => 0.05,
        CritterSpecies::Beetle => 0.03,
    }
}

fn random_critter_path(rng: &mut impl Rng) -> [Vec2; 4] {
    let enter_side = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
    [
        Vec2::new(enter_side * 500.0, rng.gen_range(-100.0..200.0)),
        Vec2::new(enter_side * 200.0, rng.gen_range(0.0..250.0)),
        Vec2::new(-enter_side * 150.0, rng.gen_range(-50.0..200.0)),
        Vec2::new(-enter_side * 500.0, rng.gen_range(-100.0..200.0)),
    ]
}

fn manual_smoke_path(species: CritterSpecies) -> [Vec2; 4] {
    match species {
        CritterSpecies::Butterfly => [
            Vec2::new(-420.0, 90.0),
            Vec2::new(-120.0, 230.0),
            Vec2::new(120.0, 150.0),
            Vec2::new(420.0, 60.0),
        ],
        CritterSpecies::Beetle => [
            Vec2::new(420.0, -120.0),
            Vec2::new(180.0, 20.0),
            Vec2::new(-120.0, -20.0),
            Vec2::new(-420.0, -110.0),
        ],
    }
}
