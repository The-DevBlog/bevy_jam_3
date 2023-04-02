use bevy::prelude::*;

pub mod hud_cmps;
mod hud_sys;

use hud_sys::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_health_bar, spawn_stamina_bar))
            .add_system(update_stamina)
            .add_system(update_health);
    }
}
