use bevy::prelude::*;
use crate::components::{Critter, CritterSpecies};
use crate::resources::BehaviorSignals;
use crate::events::{CritterArrived, CritterDeparted};
use rand::Rng;

pub fn critter_spawner_system(
    mut commands: Commands,
    behavior: Res<BehaviorSignals>,
    existing_critters: Query<&Critter>,
    time: Res<Time>,
    mut spawn_timer: Local<f32>,
    asset_server: Res<AssetServer>,
    mut events: EventWriter<CritterArrived>,
) {
    *spawn_timer += time.delta_secs();

    // Butterfly: spawns after 30 minutes of focus
    if behavior.current_focus_streak_secs >= 1800.0 {
        let has_butterfly = existing_critters.iter().any(|c| c.species == CritterSpecies::Butterfly);
        if !has_butterfly && *spawn_timer > 5.0 {
            let mut rng = rand::thread_rng();
            let path = random_butterfly_path(&mut rng);

            commands.spawn((
                Sprite {
                    image: asset_server.load("critters/butterfly.png"),
                    ..default()
                },
                Transform::from_xyz(path[0].x, path[0].y, 40.0),
                Critter {
                    species: CritterSpecies::Butterfly,
                    path_progress: 0.0,
                    speed: 0.05,
                    path,
                },
            ));

            events.send(CritterArrived { species: CritterSpecies::Butterfly });
            *spawn_timer = 0.0;
        }
    }

    // Beetle: 5% chance per minute when active
    if behavior.is_active && *spawn_timer > 60.0 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.05 {
            let has_beetle = existing_critters.iter().any(|c| c.species == CritterSpecies::Beetle);
            if !has_beetle {
                let path = random_butterfly_path(&mut rng);

                commands.spawn((
                    Sprite {
                        image: asset_server.load("critters/beetle.png"),
                        ..default()
                    },
                    Transform::from_xyz(path[0].x, path[0].y, 40.0),
                    Critter {
                        species: CritterSpecies::Beetle,
                        path_progress: 0.0,
                        speed: 0.03,
                        path,
                    },
                ));

                events.send(CritterArrived { species: CritterSpecies::Beetle });
            }
        }
        *spawn_timer = 0.0;
    }
}

fn random_butterfly_path(rng: &mut impl Rng) -> [Vec2; 4] {
    let enter_side = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
    [
        Vec2::new(enter_side * 500.0, rng.gen_range(-100.0..200.0)),
        Vec2::new(enter_side * 200.0, rng.gen_range(0.0..250.0)),
        Vec2::new(-enter_side * 150.0, rng.gen_range(-50.0..200.0)),
        Vec2::new(-enter_side * 500.0, rng.gen_range(-100.0..200.0)),
    ]
}
