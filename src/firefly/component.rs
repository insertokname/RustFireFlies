use bevy::prelude::*;

#[derive(Component)]
pub struct Firefly {
    pub speed: f32,
    pub direction: Vec2,
    /// How much is the firefly charged, when charge_ammount reaches
    /// FIREFLY_MAX_CHARGE the firefly lights up
    pub charge_amount: f32,
    /// Will go from FIREFLY_MAX_INTENSITY to 0, and will be used as
    /// parameter for linear interpolation between FIREFLY_BASE_COLOR
    /// and FIREFLY_LIGHTUP_COLOR
    pub light_intensity: f32
}
