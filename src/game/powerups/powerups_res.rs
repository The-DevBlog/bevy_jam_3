use bevy::prelude::*;

use super::POWERUP_SPAWN_TIME;

#[derive(Resource)]
pub struct PowerUpSpawnTime(pub Timer);

impl Default for PowerUpSpawnTime {
    fn default() -> Self {
        PowerUpSpawnTime(Timer::from_seconds(
            POWERUP_SPAWN_TIME,
            TimerMode::Repeating,
        ))
    }
}
