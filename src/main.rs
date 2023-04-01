use bevy::prelude::*;

mod game;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GamePlugin)
        .run();
}
