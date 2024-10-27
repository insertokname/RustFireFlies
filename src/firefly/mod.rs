pub mod component;
pub mod systems;

pub const FIREFLY_RADIUS: f32 = 10.;

pub const FIREFLY_COUNT: i32 = 100;

pub const FIREFLY_MIN_SPEED: f32 = 1.;
pub const FIREFLY_MAX_SPEED: f32 = 1.;
const _: () = assert!(
    FIREFLY_MIN_SPEED <= FIREFLY_MAX_SPEED,
    "'FIREFLY_MIN_SPEED' must be smaller than 'FIREFLY_MAX_SPEED'"
);
