use bevy::prelude::*;

pub mod projectile_cmps;
mod projectile_sys;

use projectile_sys::*;

use crate::AppState;

pub const PROJECTILE_SPEED: f32 = 15.0;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                shoot_gamepad,
                move_projectile,
                dmg_enemy,
                despawn_projectile,
            )
                .in_set(OnUpdate(AppState::Game)),
        );
    }
}
