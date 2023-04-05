use bevy::prelude::*;

pub mod player_cmps;
pub mod player_sys;

use player_sys::*;

use crate::AppState;

pub const PLAYER_SPEED: f32 = 2.5;
pub const PLAYER_HP: f32 = 100.0;
pub const PLAYER_STAMINA: f32 = 50.0;
pub const SPRINT_SPEED: f32 = 2.0;
pub const PLAYER_SIZE: f32 = 0.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    move_player_keyboard,
                    move_player_gamepad,
                    update_stamina,
                    update_health,
                )
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}
