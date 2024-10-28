mod firefly;
mod fps;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::{PresentMode, WindowResolution}};

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
            resolution: WindowResolution::new(800., 800.).with_scale_factor_override(1.),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_systems(Startup, firefly::systems::spawn_fireflies)
    .add_systems(Update, firefly::systems::clamp_on_resize)
    .add_systems(Update, firefly::systems::scramble_fireflies)
    .add_systems(Update, firefly::systems::light_manager)
    .add_systems(FixedUpdate, firefly::systems::movement)
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(Startup, setup_camera)
    .add_plugins(fps::FpsPlugin);
    app.run();
}
