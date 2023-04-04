use bevy::prelude::*;

pub mod enemy_cmps;
pub mod enemy_res;
mod enemy_sys;

use enemy_res::*;
use enemy_sys::*;

use crate::AppState;

pub const ENEMY_SPAWN_TIME: f32 = 1.5;
pub const ENEMY_SPEED: f32 = 2.5;
pub const ENEMY_HP: f32 = 100.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>().add_systems(
            (spawn_enemies, enemy_tracking, enemy_collision).in_set(OnUpdate(AppState::Game)),
        );
    }
}
