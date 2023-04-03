use bevy::prelude::*;

#[derive(Component)]
pub struct Hp(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Damage {
    pub original: f32,
    pub current: f32,
}

impl Damage {
    pub fn new(original: f32) -> Self {
        Self {
            original,
            current: original,
        }
    }
}
