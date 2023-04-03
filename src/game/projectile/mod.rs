use bevy::prelude::*;

pub mod projectile_cmps;
mod projectile_sys;

use projectile_sys::*;

pub const PROJECTILE_SPEED: f32 = 15.0;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot_gamepad)
            .add_system(move_projectile)
            .add_system(damage_enemy)
            .add_system(despawn_projectile);
    }
}
