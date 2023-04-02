use bevy::prelude::*;

pub mod enemy_cmps;
mod enemy_sys;

use enemy_sys::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}
