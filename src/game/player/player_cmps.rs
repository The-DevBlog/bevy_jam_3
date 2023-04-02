use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Stamina {
    pub max: f32,
    pub current: f32,
    pub regen_time: Timer,
}

impl Stamina {
    pub fn new(max: f32) -> Self {
        Self {
            max,
            current: max,
            regen_time: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct IsSprinting(pub bool);
