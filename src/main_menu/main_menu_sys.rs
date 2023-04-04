use bevy::prelude::*;

use crate::{gamepad::gamepad_rcs::MyGamepad, AppState};

use super::{
    main_menu_cmps::{MainMenu, MainMenuCamera, PlayBtn},
    MENU_COLOR, PLAY_BTN_COLOR, PLAY_BTN_COLOR_HOVER,
};

pub fn spawn_menu(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((Camera3dBundle::default(), MainMenuCamera));

    let container = (
        NodeBundle {
            background_color: MENU_COLOR.into(),
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        },
        MainMenu,
        Name::new("Main Menu"),
    );

    let play_btn = (
        ButtonBundle {
            background_color: PLAY_BTN_COLOR.into(),
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(75.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        PlayBtn,
        Name::new("Play Button"),
    );

    let play_txt = (
        TextBundle::from_section(
            "Play - A",
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                ..default()
            },
        ),
        Name::new("Play Text"),
    );

    cmds.spawn(container).with_children(|parent| {
        parent.spawn(play_btn).with_children(|parent| {
            parent.spawn(play_txt);
        });
    });
}

pub fn despawn_menu(
    mut cmds: Commands,
    main_menu_cam_q: Query<Entity, With<MainMenuCamera>>,
    main_menu_q: Query<Entity, With<MainMenu>>,
) {
    if let Ok(camera) = main_menu_cam_q.get_single() {
        cmds.entity(camera).despawn_recursive();
    }

    if let Ok(menu) = main_menu_q.get_single() {
        cmds.entity(menu).despawn_recursive();
    }
}

pub fn select_play_gamepad(
    btns: Res<Input<GamepadButton>>,
    cur_app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::South)))
        .unwrap_or(false);

    if gamepad_input {
        if cur_app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn select_play_mouse(
    mut interact_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayBtn>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_clr) in &mut interact_q {
        match *interaction {
            Interaction::Clicked => next_app_state.set(AppState::Game),
            Interaction::Hovered => *background_clr = PLAY_BTN_COLOR_HOVER.into(),
            Interaction::None => *background_clr = PLAY_BTN_COLOR.into(),
        }
    }
}
