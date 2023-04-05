use bevy::prelude::*;

#[derive(Component)]
pub struct Hp(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Damage {
    pub value: f32,
    pub current: f32,
}

impl Damage {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            current: value,
        }
    }
}

#[derive(Component)]
pub struct Game;
