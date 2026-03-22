use crate::components::{Critter, CritterPathId, CritterSpecies};
use crate::events::CritterArrived;
use crate::resources::{BehaviorSignals, DebugActions, DebugTelemetry, SceneArtCatalog};
use crate::systems::setup::SceneAssetHandles;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use rand::Rng;

#[derive(SystemParam)]
pub struct CritterSpawnContext<'w, 's> {
    commands: Commands<'w, 's>,
    time: Res<'w, Time>,
    actions: ResMut<'w, DebugActions>,
    telemetry: ResMut<'w, DebugTelemetry>,
    events: EventWriter<'w, CritterArrived>,
    scene_assets: Res<'w, SceneAssetHandles>,
    art_catalog: Res<'w, SceneArtCatalog>,
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
            let (path_id, path) = manual_smoke_path(species);
            spawn_critter(
                &mut ctx.commands,
                species,
                path_id,
                path,
                critter_speed(species) * 3.2,
                &ctx.scene_assets,
                &ctx.art_catalog,
            );
            ctx.events.send(CritterArrived { species });
        }
    }

    if behavior.current_focus_streak_secs >= 1800.0 {
        let has_butterfly = existing_critters
            .iter()
            .any(|c| c.species == CritterSpecies::Butterfly);
        if !has_butterfly && *ctx.spawn_timer > 5.0 {
            let mut rng = rand::thread_rng();
            let (path_id, path) = random_path(&mut rng, CritterSpecies::Butterfly);
            spawn_critter(
                &mut ctx.commands,
                CritterSpecies::Butterfly,
                path_id,
                path,
                critter_speed(CritterSpecies::Butterfly),
                &ctx.scene_assets,
                &ctx.art_catalog,
            );
            ctx.events.send(CritterArrived {
                species: CritterSpecies::Butterfly,
            });
            *ctx.spawn_timer = 0.0;
        }
    }

    if behavior.is_active && *ctx.spawn_timer > 60.0 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.05 {
            let has_beetle = existing_critters
                .iter()
                .any(|c| c.species == CritterSpecies::Beetle);
            if !has_beetle {
                let (path_id, path) = random_path(&mut rng, CritterSpecies::Beetle);
                spawn_critter(
                    &mut ctx.commands,
                    CritterSpecies::Beetle,
                    path_id,
                    path,
                    critter_speed(CritterSpecies::Beetle),
                    &ctx.scene_assets,
                    &ctx.art_catalog,
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
    species: CritterSpecies,
    path_id: CritterPathId,
    path: [Vec3; 4],
    speed: f32,
    assets: &SceneAssetHandles,
    art_catalog: &SceneArtCatalog,
) {
    let material = match species {
        CritterSpecies::Butterfly => assets.butterfly_material.clone(),
        CritterSpecies::Beetle => assets.beetle_material.clone(),
    };

    let root = commands
        .spawn((
            Transform::from_translation(path[0]),
            Visibility::Visible,
            Critter {
                species,
                path_progress: 0.0,
                speed,
                path,
                path_id,
            },
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        let custom_scene = match species {
            CritterSpecies::Butterfly => art_catalog.butterfly_scene.clone(),
            CritterSpecies::Beetle => art_catalog.beetle_scene.clone(),
        };

        if let Some(scene) = custom_scene {
            parent.spawn((SceneRoot(scene), Transform::from_scale(Vec3::splat(1.0))));
            return;
        }

        match species {
            CritterSpecies::Butterfly => {
                parent.spawn((
                    Mesh3d(assets.sphere_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_scale(Vec3::new(0.16, 0.12, 0.12)),
                ));
                parent.spawn((
                    Mesh3d(assets.capsule_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(-0.13, 0.02, 0.0)
                        .with_rotation(Quat::from_rotation_z(1.1))
                        .with_scale(Vec3::new(0.06, 0.24, 0.02)),
                ));
                parent.spawn((
                    Mesh3d(assets.capsule_mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(0.13, 0.02, 0.0)
                        .with_rotation(Quat::from_rotation_z(-1.1))
                        .with_scale(Vec3::new(0.06, 0.24, 0.02)),
                ));
            }
            CritterSpecies::Beetle => {
                parent.spawn((
                    Mesh3d(assets.sphere_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_scale(Vec3::new(0.22, 0.14, 0.18)),
                ));
                parent.spawn((
                    Mesh3d(assets.sphere_mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(0.0, 0.05, 0.08).with_scale(Vec3::new(0.13, 0.09, 0.1)),
                ));
            }
        }
    });
}

fn critter_speed(species: CritterSpecies) -> f32 {
    match species {
        CritterSpecies::Butterfly => 0.075,
        CritterSpecies::Beetle => 0.055,
    }
}

fn random_path(rng: &mut impl Rng, species: CritterSpecies) -> (CritterPathId, [Vec3; 4]) {
    match species {
        CritterSpecies::Butterfly => (
            CritterPathId::CanopyDrift,
            [
                Vec3::new(-2.1, 1.5 + rng.gen_range(-0.2..0.25), 0.85),
                Vec3::new(-0.85, 2.35 + rng.gen_range(-0.15..0.2), -0.2),
                Vec3::new(0.95, 1.95 + rng.gen_range(-0.2..0.2), 0.7),
                Vec3::new(2.15, 1.25 + rng.gen_range(-0.2..0.25), 0.9),
            ],
        ),
        CritterSpecies::Beetle => (
            CritterPathId::SoilTraverse,
            [
                Vec3::new(1.7, 0.24, 0.7),
                Vec3::new(0.65, 0.28, 0.18),
                Vec3::new(-0.55, 0.21, -0.08),
                Vec3::new(-1.6, 0.26, 0.55),
            ],
        ),
    }
}

fn manual_smoke_path(species: CritterSpecies) -> (CritterPathId, [Vec3; 4]) {
    match species {
        CritterSpecies::Butterfly => (
            CritterPathId::GlassSweep,
            [
                Vec3::new(-1.8, 1.1, 1.75),
                Vec3::new(-0.7, 2.35, 1.35),
                Vec3::new(0.65, 2.0, 1.45),
                Vec3::new(1.8, 1.2, 1.75),
            ],
        ),
        CritterSpecies::Beetle => (
            CritterPathId::SoilTraverse,
            [
                Vec3::new(1.55, 0.24, 0.6),
                Vec3::new(0.45, 0.29, 0.2),
                Vec3::new(-0.45, 0.22, 0.05),
                Vec3::new(-1.4, 0.24, 0.58),
            ],
        ),
    }
}
