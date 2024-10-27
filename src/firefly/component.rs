use bevy::prelude::*;

#[derive(Component)]
pub struct Firefly {
    pub speed: f32,
    pub direction: Vec2,
}
