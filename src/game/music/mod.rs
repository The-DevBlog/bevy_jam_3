use bevy::prelude::*;

use crate::AppState;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_music.in_schedule(OnEnter(AppState::Game)));
    }
}

pub fn play_music(assets: Res<AssetServer>, audio: Res<Audio>) {
    let music = assets.load(r"audio\music\tvs_story.ogg");
    audio.play(music);
}
