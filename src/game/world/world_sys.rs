use bevy::{prelude::*, render::render_resource::Face};
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::game::game_cmps::Game;

use super::{
    world_res::{Colors, LightTimer},
    MAP_SIZE, WALL_HEIGHT,
};

pub fn spawn_ground(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_txtr = assets.load("textures/checkers03_s.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(floor_txtr.clone()),
        ..default()
    });

    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: MAP_SIZE,
                ..default()
            })),
            material: material_handle,
            ..default()
        },
        Game,
        Collider::cuboid(MAP_SIZE / 2.0, 0.0, MAP_SIZE / 2.0),
        Name::new("Ground"),
    ));
}

pub fn spawn_walls(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let north_wall = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(MAP_SIZE, WALL_HEIGHT),
                ..default()
            })),
            material: materials.add(Color::CRIMSON.into()),
            transform: Transform {
                translation: Vec3::new(0.0, WALL_HEIGHT / 2.0, -MAP_SIZE / 2.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, 0.0),
        Game,
        Name::new("North Wall"),
    );

    let south_wall = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(MAP_SIZE, WALL_HEIGHT),
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::CRIMSON.into(),
                cull_mode: Some(Face::Front),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, WALL_HEIGHT / 2.0, MAP_SIZE / 2.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, 0.0),
        Game,
        Name::new("South Wall"),
    );

    let east_wall = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(MAP_SIZE, WALL_HEIGHT),
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::CRIMSON.into(),
                cull_mode: Some(Face::Front),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, 0.0),
                rotation: Quat::from_rotation_y(PI / 2.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, 0.0),
        Game,
        Name::new("East Wall"),
    );

    let west_wall = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(MAP_SIZE, WALL_HEIGHT),
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::CRIMSON.into(),
                cull_mode: Some(Face::Back),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(-MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, 0.0),
                rotation: Quat::from_rotation_y(PI / 2.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, 1.0, 0.0),
        Game,
        Name::new("West Wall"),
    );

    cmds.spawn(north_wall);
    cmds.spawn(south_wall);
    cmds.spawn(east_wall);
    cmds.spawn(west_wall);
}

pub fn spawn_light(mut cmds: Commands) {
    cmds.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                color: Color::GREEN.into(),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Game,
        Name::new("Point Light"),
    ));
}

pub fn change_light_clr(
    mut light_q: Query<&mut PointLight, With<PointLight>>,
    mut light_timer: ResMut<LightTimer>,
    time: Res<Time>,
    colors: Res<Colors>,
) {
    light_timer.0.tick(time.delta());
    if let Ok(mut light) = light_q.get_single_mut() {
        if light_timer.0.finished() {
            let rng = rand::thread_rng().gen_range(0..colors.0.len());
            light.color = colors.0[rng].into();
        }
    }
}
