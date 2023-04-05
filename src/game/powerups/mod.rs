use bevy::prelude::*;

pub mod powerups_cmps;
pub mod powerups_res;
mod powerups_sys;

use powerups_res::*;
use powerups_sys::*;

use crate::AppState;

pub const POWERUP_SPAWN_TIME: f32 = 2.0;
pub const DMG_BOOST: f32 = 25.0;
pub const HP_BOOST: f32 = 20.0;

pub struct PowerUpsPlugin;

impl Plugin for PowerUpsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerUpSpawnTime>()
            .init_resource::<DamageDuration>()
            .add_systems(
                (
                    spawn_powerups,
                    collect_stamina_powerup,
                    collect_dmg_powerup,
                    collect_hp_powerup,
                    tick_dmg_duration_timer,
                )
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}
