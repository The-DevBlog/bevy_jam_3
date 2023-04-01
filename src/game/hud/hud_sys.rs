use bevy::prelude::*;

fn txt(txt: &str, name: &str, assets: &Res<AssetServer>) -> (TextBundle, Name) {
    (
        TextBundle::from_section(
            txt.to_string(),
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
            },
        ),
        Name::new(name.to_string()),
    )
}

fn container(color: Color, position_left: Val, name: &str) -> (NodeBundle, Name) {
    (
        NodeBundle {
            background_color: color.into(),
            style: Style {
                align_self: AlignSelf::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                position: UiRect::left(position_left),
                size: Size::height(Val::Px(45.0)),
                ..default()
            },
            ..default()
        },
        Name::new(name.to_string()),
    )
}

pub fn spawn_health_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = container(Color::RED, Val::Percent(0.0), "Health Bar");
    let txt = txt("HEALTH", "Health Bar Text", &assets);

    cmds.spawn(container).with_children(|parent| {
        parent.spawn(txt);
    });
}

pub fn spawn_stamina_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = container(Color::DARK_GREEN, Val::Percent(81.5), "Stamina Bar");
    let txt = txt("STAMINA", "Stamina Bar Text", &assets);

    cmds.spawn(container).with_children(|parent| {
        parent.spawn(txt);
    });
}
