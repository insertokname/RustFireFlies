use super::*;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use component::Firefly;
use rand::{self, distributions::uniform::SampleRange, Rng};

fn generate_random_direction() -> Vec2 {
    let mut rng = rand::thread_rng();
    Vec2 {
        x: rng.gen_range(-1.0..=1.0),
        y: rng.gen_range(-1.0..=1.0),
    }
    .normalize()
}

fn generate_random_speed() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(FIREFLY_MIN_SPEED..=FIREFLY_MAX_SPEED)
}

fn generate_random_transform(
    x_range: impl SampleRange<f32>,
    y_range: impl SampleRange<f32>,
) -> Transform {
    let mut rng = rand::thread_rng();
    Transform::from_xyz(rng.gen_range(x_range), rng.gen_range(y_range), 0.)
}

pub fn spawn_fireflies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let width = window.physical_width() as f32 / 2.;
    let width_range = (-width + 1.)..width;
    let height = window.physical_height() as f32 / 2.;
    let height_range = (-height + 1.)..height;

    let firefly_mesh = Mesh2dHandle(meshes.add(Circle::new(FIREFLY_RADIUS)));
    let firefly_material = materials.add(Color::linear_rgb(1.0, 1.0, 0.0));

    for _ in 0..FIREFLY_COUNT {
        commands.spawn((
            ColorMesh2dBundle {
                mesh: firefly_mesh.clone(),
                material: firefly_material.clone(),
                // transform: Transform::from_xyz(1920./2., 0., 0.),
                transform: generate_random_transform(width_range.clone(), height_range.clone()),
                ..default()
            },
            Firefly {
                speed: generate_random_speed(),
                direction: generate_random_direction(),
            },
        ));
    }
}

pub fn movement(mut firefly_query: Query<(&mut Firefly, &mut Transform)>, windows: Query<&Window>) {
    let window = windows.single();
    let window_size = window.size();
    let window_max = window_size / 2.;
    let window_min = window_max-window_size;

    for (firefly, mut transform) in &mut firefly_query {
        transform.translation += (firefly.direction * firefly.speed).extend(0.);
        transform.translation = transform
            .translation
            .clamp(window_min.extend(0.), window_max.extend(0.));
    }
}
