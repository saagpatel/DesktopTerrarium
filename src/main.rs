use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResizeConstraints, WindowResolution};
use desktop_terrarium::plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Desktop Terrarium".to_string(),
                resolution: WindowResolution::new(800, 600),
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                resize_constraints: WindowResizeConstraints {
                    min_width: 400.0,
                    min_height: 300.0,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(plugins::TerrariumPlugin)
        .run();
}
