use bevy::prelude::*;

use crate::{gamepad::gamepad_rcs::MyGamepad, AppState};

use super::{game_cmps::Game, game_res::GameTime};

pub fn exit_game(
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_time: ResMut<GameTime>,
) {
    // gamepad
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::Start)))
        .unwrap_or(false);

    if gamepad_input {
        next_app_state.set(AppState::MainMenu);
        game_time.0.reset(); // reset stopwatch
    }
}

pub fn game_over(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::GameOver);
}

pub fn despawn_game(mut cmds: Commands, all_q: Query<Entity, With<Game>>) {
    for ent in all_q.iter() {
        cmds.entity(ent).despawn_recursive();
    }
}
