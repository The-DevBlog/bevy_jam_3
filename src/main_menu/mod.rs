use bevy::prelude::*;

pub mod main_menu_cmps;
mod main_menu_sys;

use main_menu_sys::*;

use crate::AppState;

pub const MENU_COLOR: Color = Color::rgb(0.22, 0.25, 0.31);
pub const PLAY_BTN_COLOR: Color = Color::GRAY;
pub const PLAY_BTN_COLOR_HOVER: Color = Color::rgb(0.65, 0.65, 0.65);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems(
                (select_play_gamepad, select_play_mouse).in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}
