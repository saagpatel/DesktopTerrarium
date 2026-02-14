use bevy::prelude::*;
use crate::components::RainDrop;
use crate::resources::WeatherState;
use crate::resources::WeatherType;
use rand::Rng;

const RAIN_SPAWN_RATE: f32 = 15.0; // drops per second

#[derive(Resource)]
pub struct RainAssets {
    pub raindrop_handle: Handle<Image>,
    pub spawn_timer: Timer,
}

pub fn setup_rain_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RainAssets {
        raindrop_handle: asset_server.load("particles/raindrop.png"),
        spawn_timer: Timer::from_seconds(1.0 / RAIN_SPAWN_RATE, TimerMode::Repeating),
    });
}

pub fn rain_spawn_system(
    mut commands: Commands,
    mut rain_assets: ResMut<RainAssets>,
    weather: Res<WeatherState>,
    time: Res<Time>,
) {
    // Only spawn during Rain weather
    if weather.current != WeatherType::Rain && weather.target != WeatherType::Rain {
        return;
    }

    // Adjust spawn rate based on transition
    let spawn_multiplier = if weather.current == WeatherType::Rain {
        if weather.target == WeatherType::Rain {
            1.0 // Fully raining
        } else {
            1.0 - weather.transition_progress // Fading out
        }
    } else {
        weather.transition_progress // Fading in
    };

    if spawn_multiplier <= 0.0 {
        return;
    }

    rain_assets.spawn_timer.tick(time.delta());

    if rain_assets.spawn_timer.just_finished() {
        let mut rng = rand::thread_rng();

        // Spawn rain drops based on multiplier
        if rng.gen::<f32>() < spawn_multiplier {
            commands.spawn((
                Sprite {
                    image: rain_assets.raindrop_handle.clone(),
                    ..default()
                },
                Transform::from_xyz(
                    rng.gen_range(-420.0..420.0),
                    320.0,
                    50.0,
                ),
                RainDrop {
                    velocity: Vec2::new(
                        rng.gen_range(-10.0..10.0),
                        rng.gen_range(-300.0..-200.0),
                    ),
                    lifetime: 3.0,
                },
            ));
        }
    }
}

pub fn rain_update_system(
    mut commands: Commands,
    mut rain: Query<(Entity, &mut RainDrop, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut drop, mut transform) in &mut rain {
        // Move the raindrop
        transform.translation.x += drop.velocity.x * time.delta_secs();
        transform.translation.y += drop.velocity.y * time.delta_secs();

        // Update lifetime
        drop.lifetime -= time.delta_secs();

        // Despawn if expired or below window
        if drop.lifetime <= 0.0 || transform.translation.y < -320.0 {
            commands.entity(entity).despawn();
        }
    }
}

// Entity count safety check
pub fn rain_limit_system(
    rain: Query<Entity, With<RainDrop>>,
) {
    if rain.iter().count() > 200 {
        warn!("Rain particle count exceeded 200, consider reducing spawn rate");
    }
}
