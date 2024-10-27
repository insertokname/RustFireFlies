mod firefly;
mod fps;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};

fn setup_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    // camera.projection.scaling_mode = ScalingMode::AutoMin {
    //     min_width: 1920.,
    //     min_height: 1080.,
    // };

    commands.spawn(camera);
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_systems(Startup, firefly::systems::spawn_fireflies)
    .add_systems(FixedUpdate, (firefly::systems::movement,))
    .add_systems(Startup, setup_camera)
    .add_plugins(fps::FpsPlugin);
    app.run();
}
