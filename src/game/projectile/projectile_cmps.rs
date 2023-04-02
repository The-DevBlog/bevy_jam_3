use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec3,
    pub damage: f32,
}
