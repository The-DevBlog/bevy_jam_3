use bevy::prelude::*;

mod projectile_sys;

use projectile_sys::*;

pub const PROJECTILE_SPEED: f32 = 8.0;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot_gamepad)
            .add_system(projectile_movement);
    }
}
