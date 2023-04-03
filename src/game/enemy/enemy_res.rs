use bevy::prelude::*;

use super::ENEMY_SPAWN_TIME;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}
