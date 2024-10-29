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

fn generate_random_charge() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..FIREFLY_MAX_CHARGE)
}

fn get_screen_bounds(screen_size: Vec2) -> (Vec2, Vec2) {
    let mut window_max = screen_size / 2.;
    let window_min = (window_max - screen_size) + FIREFLY_RADIUS;
    window_max -= FIREFLY_RADIUS;
    (window_min, window_max)
}

fn get_screen_ranges(
    screen_size: Vec2,
) -> (impl SampleRange<f32> + Clone, impl SampleRange<f32> + Clone) {
    let (window_min, window_max) = get_screen_bounds(screen_size);
    let width_range = (window_min.x + FIREFLY_RADIUS + 1.)..(window_max.x - FIREFLY_RADIUS);
    let height_range = (window_min.y + FIREFLY_RADIUS + 1.)..(window_max.y - FIREFLY_RADIUS);
    (width_range, height_range)
}

pub fn spawn_fireflies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let (width_range, height_range) = get_screen_ranges(window.size());
    for _ in 0..FIREFLY_COUNT {
        let firefly_mesh = Mesh2dHandle(meshes.add(Circle::new(FIREFLY_RADIUS)));
        let firefly_material = materials.add(FIREFLY_BASE_COLOR);
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
                charge_amount: generate_random_charge(),
                light_intensity: 0.,
            },
        ));
    }
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

pub fn scramble_fireflies(
    mut firefly_query: Query<&mut Transform, With<Firefly>>,
    windows: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let window = windows.single();
        let (width_range, height_range) = get_screen_ranges(window.size());

        for mut transform in &mut firefly_query {
            *transform = generate_random_transform(width_range.clone(), height_range.clone());
        }
    }
}

#[derive(Event)]
pub struct LightUpEvent {
    entity: Entity,
}

pub fn light_manager(
    mut firefly_query: Query<(&mut Firefly, &mut Handle<ColorMaterial>, Entity)>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_lightup: EventWriter<LightUpEvent>,
) {
    for (mut firefly, color, entity) in &mut firefly_query {
        let color: &mut ColorMaterial = materials.get_mut(color.id()).unwrap();
        if firefly.charge_amount >= FIREFLY_MAX_CHARGE {
            firefly.light_intensity = FIREFLY_MAX_INTENSITY;
            firefly.charge_amount = 0.;
            ev_lightup.send(LightUpEvent { entity: entity });
        }
        firefly.charge_amount += FIREFLY_CHARGE_ADD * time.delta_seconds();

        if firefly.light_intensity > 0. {
            color.color = FIREFLY_BASE_COLOR.mix(
                &FIREFLY_LIGHTUP_COLOR,
                firefly.light_intensity / FIREFLY_MAX_INTENSITY,
            );
            firefly.light_intensity -= FIREFLY_INTENSITY_DISCHARGE_RATE * time.delta_seconds();
        }
    }
}

pub fn add_impulse_neighbours(
    mut ev_lightup: EventReader<LightUpEvent>,
    mut firefly_query: Query<(&mut Firefly, &Transform, Entity)>,
) {
    for event in ev_lightup.read() {
        let cur_transform = if let Ok(ok) = firefly_query.get(event.entity) {
            ok.1.clone()
        } else {
            log::error!(
                "Error when trying to procces lightup event! Couldn't get entity {}",
                event.entity
            );
            break;
        };

        for (mut firefly, transform, entity) in &mut firefly_query {
            if entity != event.entity
                && cur_transform.translation.distance(transform.translation)
                    <= FIREFLY_NEIGHBOUR_DISTANCE
                && firefly.light_intensity <= 0.
            {
                firefly.charge_amount += FIREFLY_NEIGHBOUR_IMPULSE_AMOUNT;
            }
        }
    }
}
