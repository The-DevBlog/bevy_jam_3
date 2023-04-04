use bevy::prelude::*;

pub mod hud_cmps;
mod hud_sys;

use hud_sys::*;

use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_health_bar, spawn_stamina_bar).in_schedule(OnEnter(AppState::Game)))
            .add_systems((update_stamina_bar, update_health_bar).in_set(OnUpdate(AppState::Game)));
    }
}
