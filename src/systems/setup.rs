use crate::components::{
    LightRigRole, MainTerrariumCamera, Plant, PlantAnchor, PlantAnchorId, PlantSpecies,
    PlantVisualPart, WeatherReactiveSurface, WindReactive,
};
use crate::resources::{SceneArtCatalog, SceneMoodState, TerrariumScenePreset};
use bevy::math::primitives::{Capsule3d, Cuboid, Cylinder, Sphere, Torus};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

const LEFT_SUPPORT_ANCHOR: Vec3 = Vec3::new(-1.05, 0.15, 0.55);
const HERO_ANCHOR: Vec3 = Vec3::new(0.0, 0.2, 0.05);
const RIGHT_SUPPORT_ANCHOR: Vec3 = Vec3::new(1.1, 0.12, 0.45);

#[derive(Resource, Clone)]
pub struct SceneAssetHandles {
    pub sphere_mesh: Handle<Mesh>,
    pub capsule_mesh: Handle<Mesh>,
    pub cuboid_mesh: Handle<Mesh>,
    pub cylinder_mesh: Handle<Mesh>,
    pub ring_mesh: Handle<Mesh>,
    pub backdrop_material: Handle<StandardMaterial>,
    pub desk_material: Handle<StandardMaterial>,
    pub desk_trim_material: Handle<StandardMaterial>,
    pub glass_material: Handle<StandardMaterial>,
    pub glass_highlight_material: Handle<StandardMaterial>,
    pub pebble_material: Handle<StandardMaterial>,
    pub charcoal_material: Handle<StandardMaterial>,
    pub soil_material: Handle<StandardMaterial>,
    pub bark_material: Handle<StandardMaterial>,
    pub stone_material: Handle<StandardMaterial>,
    pub moss_material: Handle<StandardMaterial>,
    pub fern_material: Handle<StandardMaterial>,
    pub stem_material: Handle<StandardMaterial>,
    pub succulent_material: Handle<StandardMaterial>,
    pub butterfly_material: Handle<StandardMaterial>,
    pub beetle_material: Handle<StandardMaterial>,
    pub rain_material: Handle<StandardMaterial>,
    pub fog_material: Handle<StandardMaterial>,
    pub wind_leaf_material: Handle<StandardMaterial>,
}

pub fn setup_scene_art_catalog(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SceneArtCatalog::load(&asset_server));
}

pub fn setup_scene(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scene_mood: ResMut<SceneMoodState>,
    art_catalog: Res<SceneArtCatalog>,
) {
    scene_mood.preset = TerrariumScenePreset::ConservatoryDesk;

    ambient_light.color = scene_mood.current.ambient_color;
    ambient_light.brightness = scene_mood.current.ambient_brightness;

    let assets = SceneAssetHandles {
        sphere_mesh: meshes.add(Mesh::from(Sphere::new(0.5))),
        capsule_mesh: meshes.add(Mesh::from(Capsule3d::new(0.22, 0.9))),
        cuboid_mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
        cylinder_mesh: meshes.add(Mesh::from(Cylinder::new(0.5, 1.0))),
        ring_mesh: meshes.add(Mesh::from(Torus::new(0.68, 0.82))),
        backdrop_material: materials.add(StandardMaterial {
            base_color: scene_mood.current.backdrop_color,
            emissive: scene_mood.current.backdrop_emissive,
            perceptual_roughness: 0.96,
            ..default()
        }),
        desk_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.34, 0.26, 0.19),
            perceptual_roughness: 0.92,
            ..default()
        }),
        desk_trim_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.15, 0.1),
            perceptual_roughness: 0.86,
            ..default()
        }),
        glass_material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.88, 0.96, 0.98, 0.18),
            alpha_mode: AlphaMode::Blend,
            metallic: 0.02,
            reflectance: 0.95,
            perceptual_roughness: 0.08,
            ..default()
        }),
        glass_highlight_material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.96, 0.99, 1.0, 0.18),
            emissive: LinearRgba::rgb(0.08, 0.08, 0.09),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            ..default()
        }),
        pebble_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.53, 0.52, 0.49),
            perceptual_roughness: 0.98,
            ..default()
        }),
        charcoal_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.11),
            perceptual_roughness: 0.9,
            ..default()
        }),
        soil_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.31, 0.2, 0.12),
            perceptual_roughness: 0.95,
            ..default()
        }),
        bark_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.22, 0.15),
            perceptual_roughness: 0.9,
            ..default()
        }),
        stone_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.4, 0.38),
            perceptual_roughness: 0.97,
            ..default()
        }),
        moss_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.33, 0.18),
            perceptual_roughness: 0.82,
            ..default()
        }),
        fern_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.42, 0.23),
            perceptual_roughness: 0.72,
            ..default()
        }),
        stem_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.27, 0.14),
            perceptual_roughness: 0.78,
            ..default()
        }),
        succulent_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.36, 0.54, 0.38),
            perceptual_roughness: 0.62,
            ..default()
        }),
        butterfly_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.71, 0.37),
            emissive: LinearRgba::rgb(0.03, 0.02, 0.0),
            perceptual_roughness: 0.48,
            ..default()
        }),
        beetle_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.09, 0.05),
            perceptual_roughness: 0.34,
            metallic: 0.08,
            ..default()
        }),
        rain_material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.72, 0.85, 1.0, 0.8),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        fog_material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.86, 0.92, 0.96, scene_mood.current.haze_alpha),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        }),
        wind_leaf_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.49, 0.37, 0.18),
            perceptual_roughness: 0.75,
            cull_mode: None,
            ..default()
        }),
    };

    commands.insert_resource(assets.clone());

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(-0.35, 2.6, 8.8).looking_at(Vec3::new(0.0, 0.8, 0.15), Vec3::Y),
        MainTerrariumCamera {
            base_translation: Vec3::new(-0.35, 2.6, 8.8),
            target: Vec3::new(0.0, 0.8, 0.15),
        },
    ));

    commands.spawn((
        DirectionalLight {
            color: scene_mood.current.key_color,
            illuminance: scene_mood.current.key_illuminance,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.05, -0.72, 0.0)),
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 14.0,
            ..default()
        }
        .build(),
        LightRigRole::Key,
    ));

    commands.spawn((
        PointLight {
            color: scene_mood.current.fill_color,
            intensity: scene_mood.current.fill_intensity,
            range: 18.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 3.4, 5.4),
        LightRigRole::Fill,
    ));

    commands.spawn((
        PointLight {
            color: scene_mood.current.rim_color,
            intensity: scene_mood.current.rim_intensity,
            range: 16.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-3.5, 2.9, -4.4),
        LightRigRole::Rim,
    ));

    spawn_environment(&mut commands, &assets, &art_catalog);
    spawn_vessel(&mut commands, &assets, &art_catalog);
    spawn_hardscape(&mut commands, &assets, &art_catalog);
    spawn_plant_roots(&mut commands, &assets, &art_catalog);
}

fn spawn_environment(
    commands: &mut Commands,
    assets: &SceneAssetHandles,
    art_catalog: &SceneArtCatalog,
) {
    if let Some(scene) = art_catalog.environment_scene.clone() {
        commands.spawn((
            SceneRoot(scene),
            Transform::from_xyz(0.0, -1.55, 0.0).with_scale(Vec3::splat(1.0)),
        ));
    } else {
        commands.spawn((
            Mesh3d(assets.cuboid_mesh.clone()),
            MeshMaterial3d(assets.backdrop_material.clone()),
            Transform::from_xyz(0.0, 2.4, -7.2).with_scale(Vec3::new(18.0, 10.0, 0.25)),
        ));

        commands.spawn((
            Mesh3d(assets.cuboid_mesh.clone()),
            MeshMaterial3d(assets.desk_material.clone()),
            Transform::from_xyz(0.0, -1.55, 0.8).with_scale(Vec3::new(18.0, 0.55, 11.0)),
        ));

        commands.spawn((
            Mesh3d(assets.cuboid_mesh.clone()),
            MeshMaterial3d(assets.desk_trim_material.clone()),
            Transform::from_xyz(0.0, -1.15, 2.6).with_scale(Vec3::new(12.0, 0.15, 2.0)),
        ));
    }

    commands.spawn((
        Mesh3d(assets.sphere_mesh.clone()),
        MeshMaterial3d(assets.fog_material.clone()),
        Transform::from_xyz(0.0, 1.15, 0.15).with_scale(Vec3::new(6.6, 3.2, 3.8)),
        crate::components::HazeVolume,
    ));
}

fn spawn_vessel(
    commands: &mut Commands,
    assets: &SceneAssetHandles,
    art_catalog: &SceneArtCatalog,
) {
    if let Some(scene) = art_catalog.vessel_scene.clone() {
        commands.spawn((
            SceneRoot(scene),
            Transform::from_xyz(0.0, -1.05, 0.0).with_scale(Vec3::splat(1.0)),
            WeatherReactiveSurface,
        ));
    } else {
        commands.spawn((
            Mesh3d(assets.cylinder_mesh.clone()),
            MeshMaterial3d(assets.glass_material.clone()),
            Transform::from_xyz(0.0, 0.75, 0.0).with_scale(Vec3::new(4.65, 5.05, 4.65)),
            WeatherReactiveSurface,
        ));

        commands.spawn((
            Mesh3d(assets.ring_mesh.clone()),
            MeshMaterial3d(assets.glass_highlight_material.clone()),
            Transform::from_xyz(0.0, 3.18, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ));

        commands.spawn((
            Mesh3d(assets.cuboid_mesh.clone()),
            MeshMaterial3d(assets.glass_highlight_material.clone()),
            Transform::from_xyz(1.45, 1.18, 1.82).with_scale(Vec3::new(0.05, 3.1, 0.08)),
        ));

        commands.spawn((
            Mesh3d(assets.cuboid_mesh.clone()),
            MeshMaterial3d(assets.glass_highlight_material.clone()),
            Transform::from_xyz(-1.15, 1.45, 1.76)
                .with_rotation(Quat::from_rotation_z(0.18))
                .with_scale(Vec3::new(0.035, 2.3, 0.08)),
        ));
    }

    if let Some(scene) = art_catalog.substrate_scene.clone() {
        commands.spawn((
            SceneRoot(scene),
            Transform::from_xyz(0.0, -1.15, 0.0).with_scale(Vec3::splat(1.0)),
            WeatherReactiveSurface,
        ));
    } else {
        commands.spawn((
            Mesh3d(assets.cylinder_mesh.clone()),
            MeshMaterial3d(assets.pebble_material.clone()),
            Transform::from_xyz(0.0, -1.08, 0.0).with_scale(Vec3::new(3.4, 0.5, 3.4)),
        ));

        commands.spawn((
            Mesh3d(assets.cylinder_mesh.clone()),
            MeshMaterial3d(assets.charcoal_material.clone()),
            Transform::from_xyz(0.0, -0.76, 0.0).with_scale(Vec3::new(3.22, 0.12, 3.22)),
        ));

        commands.spawn((
            Mesh3d(assets.cylinder_mesh.clone()),
            MeshMaterial3d(assets.soil_material.clone()),
            Transform::from_xyz(0.0, -0.15, 0.0).with_scale(Vec3::new(3.55, 1.02, 3.55)),
            WeatherReactiveSurface,
        ));
    }
}

fn spawn_hardscape(
    commands: &mut Commands,
    assets: &SceneAssetHandles,
    art_catalog: &SceneArtCatalog,
) {
    if let Some(scene) = art_catalog.hardscape_scene.clone() {
        commands.spawn((
            SceneRoot(scene),
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        ));
        return;
    }

    let moss_positions = [
        Vec3::new(-1.0, 0.28, 0.7),
        Vec3::new(-0.55, 0.26, 0.42),
        Vec3::new(0.42, 0.24, 0.58),
        Vec3::new(0.95, 0.22, 0.82),
        Vec3::new(1.2, 0.2, 0.25),
    ];

    for (index, position) in moss_positions.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(assets.sphere_mesh.clone()),
            MeshMaterial3d(assets.moss_material.clone()),
            Transform::from_translation(position)
                .with_scale(Vec3::splat(0.46 + index as f32 * 0.04)),
        ));
    }

    commands.spawn((
        Mesh3d(assets.sphere_mesh.clone()),
        MeshMaterial3d(assets.stone_material.clone()),
        Transform::from_xyz(-1.45, 0.15, -0.1)
            .with_rotation(Quat::from_rotation_y(0.4))
            .with_scale(Vec3::new(0.95, 0.62, 0.72)),
    ));

    commands.spawn((
        Mesh3d(assets.sphere_mesh.clone()),
        MeshMaterial3d(assets.stone_material.clone()),
        Transform::from_xyz(1.28, 0.12, -0.16)
            .with_rotation(Quat::from_rotation_y(-0.35))
            .with_scale(Vec3::new(0.78, 0.52, 0.68)),
    ));

    commands.spawn((
        Mesh3d(assets.capsule_mesh.clone()),
        MeshMaterial3d(assets.bark_material.clone()),
        Transform::from_xyz(-0.25, 0.62, -0.35)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.28, 0.4, 1.15))
            .with_scale(Vec3::new(0.35, 1.1, 0.35)),
    ));

    commands.spawn((
        Mesh3d(assets.capsule_mesh.clone()),
        MeshMaterial3d(assets.fern_material.clone()),
        Transform::from_xyz(-1.7, 0.72, 1.25)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.45, 0.55, 0.2))
            .with_scale(Vec3::new(0.18, 1.8, 0.18)),
    ));

    commands.spawn((
        Mesh3d(assets.capsule_mesh.clone()),
        MeshMaterial3d(assets.fern_material.clone()),
        Transform::from_xyz(1.75, 0.85, 1.35)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.3, -0.35, -0.18))
            .with_scale(Vec3::new(0.15, 1.45, 0.15)),
    ));
}

fn spawn_plant_roots(
    commands: &mut Commands,
    assets: &SceneAssetHandles,
    art_catalog: &SceneArtCatalog,
) {
    let plant_layout = [
        (0_u8, LEFT_SUPPORT_ANCHOR, PlantSpecies::Fern),
        (1_u8, HERO_ANCHOR, PlantSpecies::Moss),
        (2_u8, RIGHT_SUPPORT_ANCHOR, PlantSpecies::Succulent),
    ];

    for (slot, translation, species) in plant_layout {
        let id = PlantAnchorId::from_slot(slot);
        let entity = commands
            .spawn((
                Transform::from_translation(translation),
                Visibility::Visible,
                PlantAnchor { id },
                WindReactive {
                    amplitude: 0.04 + slot as f32 * 0.01,
                    phase_offset: 0.65 * slot as f32,
                },
                Plant {
                    species,
                    stage: 0,
                    growth_progress: 0.0,
                    slot,
                },
            ))
            .id();

        respawn_plant_visual(commands, entity, species, 0, assets, art_catalog);
    }
}

pub fn respawn_plant_visual(
    commands: &mut Commands,
    plant_entity: Entity,
    species: PlantSpecies,
    stage: u8,
    assets: &SceneAssetHandles,
    art_catalog: &SceneArtCatalog,
) {
    commands.entity(plant_entity).despawn_descendants();
    commands.entity(plant_entity).with_children(|parent| {
        spawn_shadow(parent, assets);
        if let Some(scene) = art_catalog.plant_stage(species, stage) {
            parent.spawn((
                SceneRoot(scene),
                Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
                PlantVisualPart,
            ));
        } else {
            match species {
                PlantSpecies::Fern => spawn_fern(parent, stage, assets),
                PlantSpecies::Moss => spawn_moss(parent, stage, assets),
                PlantSpecies::Succulent => spawn_succulent(parent, stage, assets),
            }
        }
    });
}

fn spawn_shadow(parent: &mut ChildBuilder, assets: &SceneAssetHandles) {
    parent.spawn((
        Mesh3d(assets.cylinder_mesh.clone()),
        MeshMaterial3d(assets.charcoal_material.clone()),
        Transform::from_xyz(0.0, -0.03, 0.0).with_scale(Vec3::new(0.72, 0.02, 0.72)),
        PlantVisualPart,
    ));
}

fn spawn_fern(parent: &mut ChildBuilder, stage: u8, assets: &SceneAssetHandles) {
    let stage_scale = 0.55 + stage as f32 * 0.22;
    let frond_count = 4 + stage as usize;

    parent.spawn((
        Mesh3d(assets.capsule_mesh.clone()),
        MeshMaterial3d(assets.stem_material.clone()),
        Transform::from_xyz(0.0, 0.35 * stage_scale, 0.0).with_scale(Vec3::new(
            0.16,
            0.95 * stage_scale,
            0.16,
        )),
        PlantVisualPart,
    ));

    for index in 0..frond_count {
        let ratio = index as f32 / frond_count.max(1) as f32;
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let lift = 0.18 + ratio * 0.82 * stage_scale;
        let spread = 0.22 + ratio * 0.5 * stage_scale;
        parent.spawn((
            Mesh3d(assets.capsule_mesh.clone()),
            MeshMaterial3d(assets.fern_material.clone()),
            Transform::from_xyz(side * spread * 0.45, lift, -0.12 + ratio * 0.28)
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    -0.4 - ratio * 0.35,
                    side * (0.55 + ratio * 0.25),
                    side * (0.95 - ratio * 0.25),
                ))
                .with_scale(Vec3::new(0.1, 0.45 + ratio * 0.42 * stage_scale, 0.1)),
            PlantVisualPart,
        ));
    }
}

fn spawn_moss(parent: &mut ChildBuilder, stage: u8, assets: &SceneAssetHandles) {
    let cluster_count = 5 + stage as usize * 3;
    let radius = 0.18 + stage as f32 * 0.04;

    for index in 0..cluster_count {
        let angle = index as f32 * 0.7;
        let ring = 0.12 + (index % 3) as f32 * 0.14;
        let x = angle.cos() * ring;
        let z = angle.sin() * ring;
        let y = 0.08 + (index % 2) as f32 * 0.06 + stage as f32 * 0.03;
        parent.spawn((
            Mesh3d(assets.sphere_mesh.clone()),
            MeshMaterial3d(assets.moss_material.clone()),
            Transform::from_xyz(x, y, z).with_scale(Vec3::new(radius * 1.1, radius, radius * 1.05)),
            PlantVisualPart,
        ));
    }
}

fn spawn_succulent(parent: &mut ChildBuilder, stage: u8, assets: &SceneAssetHandles) {
    let stage_scale = 0.58 + stage as f32 * 0.15;
    let petal_count = 6 + stage as usize * 2;

    parent.spawn((
        Mesh3d(assets.sphere_mesh.clone()),
        MeshMaterial3d(assets.succulent_material.clone()),
        Transform::from_xyz(0.0, 0.16 + stage as f32 * 0.03, 0.0)
            .with_scale(Vec3::splat(0.28 + stage as f32 * 0.05)),
        PlantVisualPart,
    ));

    for index in 0..petal_count {
        let angle = index as f32 / petal_count as f32 * std::f32::consts::TAU;
        let ring = 0.25 + (index % 2) as f32 * 0.12;
        let lift = 0.12 + (index % 3) as f32 * 0.05;
        parent.spawn((
            Mesh3d(assets.capsule_mesh.clone()),
            MeshMaterial3d(assets.succulent_material.clone()),
            Transform::from_xyz(angle.cos() * ring * 0.48, lift, angle.sin() * ring * 0.48)
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    -0.95,
                    angle,
                    angle.sin() * 0.12,
                ))
                .with_scale(Vec3::new(0.11, 0.34 * stage_scale, 0.11)),
            PlantVisualPart,
        ));
    }
}
