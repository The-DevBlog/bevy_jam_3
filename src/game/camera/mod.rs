use bevy::prelude::*;

pub mod camera_cmps;
mod camera_sys;

use camera_sys::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(orbit_gamepad)
            .add_system(orbit_mouse)
            .add_system(zoom_gamepad)
            .add_system(zoom_mouse);
    }
}
