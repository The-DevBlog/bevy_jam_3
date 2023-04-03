use bevy::prelude::*;

pub mod powerups_res;
mod powerups_sys;

use powerups_res::*;
use powerups_sys::*;

pub const POWERUP_SPAWN_TIME: f32 = 2.0;

pub struct PowerUpsPlugin;

impl Plugin for PowerUpsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerUpSpawnTime>()
            .add_system(spawn_powerups);
    }
}
