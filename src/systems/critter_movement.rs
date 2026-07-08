use crate::components::Critter;
use crate::events::CritterDeparted;
use bevy::prelude::*;

pub fn critter_movement_system(
    mut commands: Commands,
    mut critters: Query<(Entity, &mut Critter, &mut Transform)>,
    time: Res<Time>,
    mut events: MessageWriter<CritterDeparted>,
) {
    for (entity, mut critter, mut transform) in &mut critters {
        // Advance along path
        critter.path_progress += critter.speed * time.delta_secs();

        if critter.path_progress >= 1.0 {
            // Path complete - despawn
            events.write(CritterDeparted {
                species: critter.species,
            });
            commands.entity(entity).despawn();
        } else {
            // Evaluate cubic Bezier curve
            let t = critter.path_progress;
            let pos = cubic_bezier(critter.path, t);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

fn cubic_bezier(points: [Vec2; 4], t: f32) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    points[0] * mt3 + points[1] * 3.0 * mt2 * t + points[2] * 3.0 * mt * t2 + points[3] * t3
}
