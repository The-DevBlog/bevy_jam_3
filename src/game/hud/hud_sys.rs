use bevy::prelude::*;

use super::hud_cmps::*;
use crate::game::player::player_cmps::*;

fn new_txt(assets: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            color: Color::WHITE,
            font: assets.load("fonts/FiraSans-Bold.ttf"),
            font_size: 25.0,
        },
    )
}

fn new_container(color: Color, position: UiRect, size: Size) -> NodeBundle {
    NodeBundle {
        background_color: color.into(),
        style: Style {
            align_self: AlignSelf::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            position,
            position_type: PositionType::Absolute,
            size,
            ..default()
        },
        ..default()
    }
}

pub fn spawn_health_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = new_container(
        Color::BLACK,
        UiRect::left(Val::Percent(1.0)),
        Size::new(Val::Px(127.5), Val::Px(42.5)),
    );

    let fill = new_container(
        Color::RED,
        UiRect::new(
            Val::Percent(1.0),
            Val::Undefined,
            Val::Percent(3.0),
            Val::Undefined,
        ),
        Size::new(Val::Px(125.0), Val::Px(40.0)),
    );

    let txt = new_txt(&assets);

    cmds.spawn((container, HealthBarContainer, Name::new("Health Bar")))
        .with_children(|parent| {
            parent.spawn(fill);
            parent.spawn((txt, HealthBarTxt, Name::new("Health Bar Text")));
        });
}

pub fn spawn_stamina_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = new_container(
        Color::BLACK,
        UiRect::right(Val::Percent(0.5)),
        Size::new(Val::Px(127.5), Val::Px(42.5)),
    );

    let fill = new_container(
        Color::DARK_GREEN,
        UiRect::new(
            Val::Undefined,
            Val::Percent(1.0),
            Val::Percent(3.0),
            Val::Undefined,
        ),
        Size::new(Val::Px(125.0), Val::Px(40.0)),
    );

    let txt = new_txt(&assets);

    cmds.spawn((container, StaminaBarContainer, Name::new("Stamina Bar")))
        .with_children(|parent| {
            parent.spawn(fill);
            parent.spawn((txt, StaminaBarTxt, Name::new("Stamina Bar Text")));
        });
}

pub fn update_stamina_bar(
    mut stamina_q: Query<&mut Text, With<StaminaBarTxt>>,
    player_q: Query<&Stamina, With<Player>>,
) {
    if let Ok(mut txt) = stamina_q.get_single_mut() {
        if let Ok(stamina) = player_q.get_single() {
            txt.sections[0].value = stamina.0.to_string();
        }
    }
}

pub fn update_health_bar(
    mut stamina_q: Query<&mut Text, With<HealthBarTxt>>,
    player_q: Query<&Health, With<Player>>,
) {
    if let Ok(mut txt) = stamina_q.get_single_mut() {
        if let Ok(health) = player_q.get_single() {
            txt.sections[0].value = health.0.to_string();
        }
    }
}
