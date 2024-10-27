use super::*;
use bevy::{prelude::*, sprite::Mesh2dHandle, window::WindowResized};
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
    let width = window.width() as f32 / 2.;
    let width_range = (-width + FIREFLY_RADIUS)..(width - FIREFLY_RADIUS);
    let height = window.height() as f32 / 2.;
    let height_range = (-height + FIREFLY_RADIUS)..(height - FIREFLY_RADIUS);

    let firefly_mesh = Mesh2dHandle(meshes.add(Circle::new(FIREFLY_RADIUS)));
    let firefly_material = materials.add(Color::linear_rgb(1.0, 1.0, 0.0));

    for _ in 0..FIREFLY_COUNT {
        commands.spawn((
            ColorMesh2dBundle {
                mesh: firefly_mesh.clone(),
                material: firefly_material.clone(),
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

fn get_screen_bounds(screen_size: Vec2) -> (Vec2, Vec2) {
    let mut window_max = screen_size / 2.;
    let window_min = (window_max - screen_size) + FIREFLY_RADIUS;
    window_max -= FIREFLY_RADIUS;
    (window_min, window_max)
}

pub fn clamp_on_resize(
    mut events: EventReader<WindowResized>,
    mut query: Query<&mut Transform, With<Firefly>>,
) {
    for event in events.read() {
        let (window_min, window_max) = get_screen_bounds(Vec2 {
            x: event.width,
            y: event.height,
        });

        for mut transform in &mut query {
            transform.translation = transform
                .translation
                .clamp(window_min.extend(0.), window_max.extend(0.));
        }
    }
}

pub fn movement(mut firefly_query: Query<(&mut Firefly, &mut Transform)>, windows: Query<&Window>) {
    let window = windows.single();
    let (window_min, window_max) = get_screen_bounds(window.size());

    for (mut firefly, mut transform) in &mut firefly_query {
        transform.translation += (firefly.direction * firefly.speed).extend(0.);
        if !(window_min.x <= transform.translation.x && transform.translation.x <= window_max.x) {
            firefly.direction.x *= -1.;
            transform.translation.x += firefly.direction.x * firefly.speed * 2.;
        }

        if !(window_min.y <= transform.translation.y && transform.translation.y <= window_max.y) {
            firefly.direction.y *= -1.;
            transform.translation.y += firefly.direction.y * firefly.speed * 2.;
        }
    }
}
