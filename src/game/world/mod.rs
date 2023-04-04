use bevy::prelude::*;

mod world_sys;

use world_sys::*;

use crate::AppState;

pub const MAP_SIZE: f32 = 25.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_ground, spawn_light).in_schedule(OnEnter(AppState::Game)));
    }
}
