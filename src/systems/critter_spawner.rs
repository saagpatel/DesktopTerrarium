use crate::components::{Critter, CritterSpecies};
use crate::events::CritterArrived;
use crate::resources::{ActivityStatus, BehaviorSignals};
use bevy::prelude::*;
use rand::{Rng, RngExt};

#[derive(Default)]
pub struct SpawnTimers {
    butterfly: f32,
    beetle: f32,
}

pub fn critter_spawner_system(
    mut commands: Commands,
    behavior: Res<BehaviorSignals>,
    existing_critters: Query<&Critter>,
    time: Res<Time>,
    mut timers: Local<SpawnTimers>,
    asset_server: Res<AssetServer>,
    mut events: MessageWriter<CritterArrived>,
) {
    timers.butterfly += time.delta_secs();
    if behavior.activity == ActivityStatus::Active {
        timers.beetle += time.delta_secs();
    }

    // Butterfly: spawns after 30 minutes of focus
    if behavior_allows_butterfly(behavior.current_focus_streak_secs) {
        let has_butterfly = existing_critters
            .iter()
            .any(|c| c.species == CritterSpecies::Butterfly);
        if !has_butterfly && timers.butterfly > 5.0 {
            let mut rng = rand::rng();
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

            events.write(CritterArrived {
                species: CritterSpecies::Butterfly,
            });
            timers.butterfly = 0.0;
        }
    }

    // Beetle: 5% chance per minute when active
    if behavior.activity == ActivityStatus::Active && timers.beetle > 60.0 {
        let mut rng = rand::rng();
        if beetle_spawn_decision(timers.beetle, true, &mut rng).unwrap_or(false) {
            let has_beetle = existing_critters
                .iter()
                .any(|c| c.species == CritterSpecies::Beetle);
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

                events.write(CritterArrived {
                    species: CritterSpecies::Beetle,
                });
            }
        }
        timers.beetle = 0.0;
    }
}

fn beetle_spawn_decision<R: Rng>(elapsed: f32, active: bool, rng: &mut R) -> Option<bool> {
    (active && elapsed > 60.0).then(|| rng.random::<f32>() < 0.05)
}

fn random_butterfly_path(rng: &mut impl Rng) -> [Vec2; 4] {
    let enter_side = if rng.random_bool(0.5) { -1.0 } else { 1.0 };
    [
        Vec2::new(enter_side * 500.0, rng.random_range(-100.0..200.0)),
        Vec2::new(enter_side * 200.0, rng.random_range(0.0..250.0)),
        Vec2::new(-enter_side * 150.0, rng.random_range(-50.0..200.0)),
        Vec2::new(-enter_side * 500.0, rng.random_range(-100.0..200.0)),
    ]
}

fn behavior_allows_butterfly(focus_secs: f64) -> bool {
    focus_secs >= 1800.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn beetle_attempts_do_not_reset_butterfly_eligibility() {
        let mut timers = SpawnTimers {
            butterfly: 6.0,
            beetle: 61.0,
        };
        let mut rng = SmallRng::seed_from_u64(1);
        let attempt_result = beetle_spawn_decision(timers.beetle, true, &mut rng);
        assert!(attempt_result.is_some());
        timers.beetle = 0.0;
        assert!(timers.butterfly > 5.0);
        assert!(behavior_allows_butterfly(1800.0));
    }
}
