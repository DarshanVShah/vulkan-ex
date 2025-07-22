use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::prelude::shape;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain);
    }
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main floating island platform
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(20.0, 1.0, 20.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(40.0, 2.0, 40.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.6, 0.3),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
    ));

    // Grass layer on top
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(38.0, 0.1, 38.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.8, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.1, 0.0),
            ..default()
        },
    ));

    // Add some decorative elements
    spawn_decorative_elements(&mut commands, &mut meshes, &mut materials);
    
    // Add some floating platforms
    spawn_floating_platforms(&mut commands, &mut meshes, &mut materials);
}

fn spawn_decorative_elements(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Trees
    for i in 0..8 {
        let angle = (i as f32) * std::f32::consts::PI * 2.0 / 8.0;
        let radius = 12.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        // Tree trunk
        commands.spawn((
            RigidBody::Fixed,
            Collider::cylinder(2.0, 0.3),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.3,
                    height: 4.0,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.4, 0.2, 0.1),
                    ..default()
                }),
                transform: Transform::from_xyz(x, 1.0, z),
                ..default()
            },
        ));

        // Tree foliage
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 2.0,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.1, 0.5, 0.1),
                    ..default()
                }),
                transform: Transform::from_xyz(x, 4.0, z),
                ..default()
            },
        ));
    }

    // Rocks
    for i in 0..12 {
        let angle = (i as f32) * std::f32::consts::PI * 2.0 / 12.0;
        let radius = 15.0 + (i % 3) as f32 * 2.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        commands.spawn((
            RigidBody::Fixed,
            Collider::ball(0.5),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                }),
                transform: Transform::from_xyz(x, 0.5, z),
                ..default()
            },
        ));
    }
}

fn spawn_floating_platforms(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Create some floating platforms around the main island
    let platform_positions = vec![
        Vec3::new(25.0, 5.0, 0.0),
        Vec3::new(-25.0, 8.0, 0.0),
        Vec3::new(0.0, 12.0, 25.0),
        Vec3::new(0.0, 6.0, -25.0),
        Vec3::new(18.0, 10.0, 18.0),
        Vec3::new(-18.0, 7.0, -18.0),
    ];
    
    for (i, pos) in platform_positions.iter().enumerate() {
        let size = 3.0 + (i % 2) as f32 * 2.0;
        
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(size, 0.5, size),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size * 2.0, 1.0, size * 2.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.6, 0.4, 0.2),
                    ..default()
                }),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                ..default()
            },
        ));
    }
} 