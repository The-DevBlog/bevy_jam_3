use bevy::prelude::*;

pub fn spawn_ground(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 10.0,
                ..default()
            })),
            material: materials.add(Color::GRAY.into()),
            ..default()
        },
        Name::new("Ground"),
    ));
}

pub fn spawn_light(mut cmds: Commands) {
    cmds.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("Point Light"),
    ));
}
