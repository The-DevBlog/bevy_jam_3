use bevy::prelude::*;

pub mod player_cmps;
mod player_sys;

use player_sys::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(keyboard_movement)
            .add_system(gamepad_movement)
            .add_system(update_stamina);
    }
}
