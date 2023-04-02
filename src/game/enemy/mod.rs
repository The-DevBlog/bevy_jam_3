use bevy::prelude::*;

pub mod enemy_cmps;
mod enemy_sys;

use enemy_sys::*;

pub const ENEMY_SPEED: f32 = 2.5;
pub const ENEMY_HP: f32 = 100.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}
