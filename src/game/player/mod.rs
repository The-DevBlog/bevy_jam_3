use bevy::prelude::*;

pub mod player_cmps;
mod player_sys;

use player_sys::*;

pub const PLAYER_SPEED: f32 = 2.5;
pub const PLAYER_HEALTH: f32 = 100.0;
pub const PLAYER_STAMINA: f32 = 50.0;
pub const SPRINT_SPEED: f32 = 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(keyboard_movement)
            .add_system(gamepad_movement)
            .add_system(update_stamina);
    }
}
