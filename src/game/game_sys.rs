use bevy::prelude::*;

use crate::AppState;

use super::{game_cmps::Game, gamepad::gamepad_rcs::MyGamepad};

pub fn exit_game(
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    // gamepad
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::Start)))
        .unwrap_or(false);

    if gamepad_input {
        next_app_state.set(AppState::MainMenu);
    }
}

pub fn despawn_game(mut cmds: Commands, all_q: Query<Entity, With<Game>>) {
    for ent in all_q.iter() {
        cmds.entity(ent).despawn_recursive();
    }
}
