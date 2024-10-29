use bevy::color::Color;

pub mod component;
pub mod systems;

pub const FIREFLY_RADIUS: f32 = 7.;
pub const FIREFLY_COUNT: i32 = 500;

pub const FIREFLY_MIN_SPEED: f32 = 0.1;
pub const FIREFLY_MAX_SPEED: f32 = 0.2;
const _: () = assert!(
    FIREFLY_MIN_SPEED <= FIREFLY_MAX_SPEED,
    "'FIREFLY_MIN_SPEED' must be smaller than 'FIREFLY_MAX_SPEED'"
);

pub const FIREFLY_MAX_CHARGE: f32 = 3.;
pub const FIREFLY_CHARGE_ADD: f32 = 1.;
pub const FIREFLY_MAX_INTENSITY: f32 = 0.75;
pub const FIREFLY_INTENSITY_DISCHARGE_RATE: f32 = 1.;

macro_rules! rgb {
    ($r:expr,$g:expr,$b:expr) => {
        Color::srgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

pub const FIREFLY_BASE_COLOR: Color = rgb!(192, 192, 192);
pub const FIREFLY_LIGHTUP_COLOR: Color = rgb!(255, 255, 0);

pub const FIREFLY_NEIGHBOUR_DISTANCE: f32 = 100.;
pub const FIREFLY_NEIGHBOUR_IMPULSE_AMOUNT: f32 = 0.1;
