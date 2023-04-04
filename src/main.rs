use bevy::prelude::*;

mod game;
pub mod gamepad;
mod main_menu;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;
use gamepad::GamepadPlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamepadPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GamePlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
