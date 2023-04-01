use bevy::prelude::*;

pub mod camera;
pub mod gamepad;
pub mod hud;
pub mod player;
pub mod world;

use camera::CameraPlugin;
use gamepad::GamepadPlugin;
use hud::HudPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GamepadPlugin)
            .add_plugin(HudPlugin);
    }
}
