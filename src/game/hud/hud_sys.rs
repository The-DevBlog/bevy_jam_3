use bevy::prelude::*;

use super::hud_cmps::*;
use crate::game::player::player_cmps::*;

fn txt(assets: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            color: Color::WHITE,
            font: assets.load("fonts/FiraSans-Bold.ttf"),
            font_size: 25.0,
        },
    )
}

fn container(color: Color, position: UiRect) -> NodeBundle {
    NodeBundle {
        background_color: color.into(),
        style: Style {
            align_self: AlignSelf::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            position,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(125.0), Val::Px(40.0)),
            ..default()
        },
        ..default()
    }
}

pub fn spawn_health_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = container(Color::RED, UiRect::left(Val::Percent(1.0)));
    let txt = txt(&assets);

    cmds.spawn((container, HealthBarContainer, Name::new("Health Bar")))
        .with_children(|parent| {
            parent.spawn((txt, HealthBarTxt, Name::new("Health Bar Text")));
        });
}

pub fn spawn_stamina_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = container(Color::DARK_GREEN, UiRect::right(Val::Percent(1.0)));
    let txt = txt(&assets);

    cmds.spawn((container, StaminaBarContainer, Name::new("Stamina Bar")))
        .with_children(|parent| {
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
