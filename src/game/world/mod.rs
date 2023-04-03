use bevy::prelude::*;

mod world_sys;

use world_sys::*;

pub const MAP_SIZE: f32 = 25.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground)
            .add_startup_system(spawn_light);
    }
}
