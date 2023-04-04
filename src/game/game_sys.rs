use bevy::prelude::*;

use crate::{gamepad::gamepad_rcs::MyGamepad, AppState};

use super::{game_cmps::Game, game_res::GameTime};

pub fn tick_game_time(mut game_time: ResMut<GameTime>, time: Res<Time>) {
    game_time.0.tick(time.delta());
    println!("TIME: {:?}", game_time.0.elapsed());
}

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

pub fn despawn_game(mut cmds: Commands, all_q: Query<Entity, With<Game>>) {
    for ent in all_q.iter() {
        cmds.entity(ent).despawn_recursive();
    }
}
