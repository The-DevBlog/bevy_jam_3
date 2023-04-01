use bevy::prelude::*;

mod world_sys;

use world_sys::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ground)
            .add_startup_system(point_light);
    }
}
