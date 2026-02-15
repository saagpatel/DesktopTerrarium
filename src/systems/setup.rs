use crate::components::{Plant, PlantSpecies, SceneLayer, TimeVariantTag};
use bevy::prelude::*;

// Plant slot positions (pixels from center)
const PLANT_SLOTS: [(f32, f32); 3] = [
    (-150.0, -80.0), // left
    (0.0, -100.0),   // center
    (160.0, -70.0),  // right
];

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Camera2d
    commands.spawn(Camera2d);

    // Backdrop layer (no time variant)
    commands.spawn((
        Sprite {
            image: asset_server.load("layers/backdrop.png"),
            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        SceneLayer {
            depth_factor: 0.0, // No parallax for backdrop
            time_variant: None,
        },
    ));

    // Glass container layer (time variant)
    // Spawn all 4 time-of-day variants at the same position
    for (phase, variant_name) in ["morning", "day", "evening", "night"].iter().enumerate() {
        let alpha = if phase == 1 { 1.0 } else { 0.0 }; // day visible by default
        commands.spawn((
            Sprite {
                image: asset_server.load(format!("layers/glass_container_{}.png", variant_name)),
                color: Color::srgba(1.0, 1.0, 1.0, alpha),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 10.0),
            SceneLayer {
                depth_factor: 0.3,
                time_variant: None, // Will be set up properly when we implement time system
            },
            TimeVariantTag {
                layer_id: 0, // glass_container
                phase: phase as u8,
            },
        ));
    }

    // Soil layer (time variant)
    for (phase, variant_name) in ["morning", "day", "evening", "night"].iter().enumerate() {
        let alpha = if phase == 1 { 1.0 } else { 0.0 }; // day visible by default
        commands.spawn((
            Sprite {
                image: asset_server.load(format!("layers/soil_{}.png", variant_name)),
                color: Color::srgba(1.0, 1.0, 1.0, alpha),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 20.0),
            SceneLayer {
                depth_factor: 0.6,
                time_variant: None, // Will be set up properly when we implement time system
            },
            TimeVariantTag {
                layer_id: 1, // soil
                phase: phase as u8,
            },
        ));
    }

    // Spawn plants
    let plant_species = [
        PlantSpecies::Fern,
        PlantSpecies::Moss,
        PlantSpecies::Succulent,
    ];
    for (slot, (x, y)) in PLANT_SLOTS.iter().enumerate() {
        let species = plant_species[slot];
        commands.spawn((
            Sprite {
                image: asset_server.load(format!("plants/{}_stage0.png", species.asset_name())),
                ..default()
            },
            Transform::from_xyz(*x, *y, 30.0),
            Plant {
                species,
                stage: 0,
                growth_progress: 0.0,
                slot: slot as u8,
            },
        ));
    }
}
