use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::prelude::shape;
use crate::camera::ThirdPersonCamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, update_camera_target)
            .add_systems(Update, ground_detection)
            .add_systems(Update, debug_player_state);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub on_ground: bool,
    pub rotation_speed: f32,
}

#[derive(Resource, Default)]
pub struct PlayerEntity(Option<Entity>);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("=== SPAWNING PLAYER ===");
    
    // Spawn player
    let player_entity = commands.spawn((
        Player {
            speed: 8.0,
            jump_force: 12.0,
            on_ground: false,
            rotation_speed: 10.0,
        },
        RigidBody::Dynamic,
        Collider::capsule_y(1.0, 0.5),
        Velocity::zero(),
        // Visual representation
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 2.0,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
    )).id();
    
    println!("Player spawned with entity ID: {:?}", player_entity);
    
    // Store the player entity in a resource
    commands.insert_resource(PlayerEntity(Some(player_entity)));
    println!("Player entity stored in resource");
}

fn update_camera_target(
    player_entity: Res<PlayerEntity>,
    mut camera_query: Query<&mut ThirdPersonCamera>,
) {
    if let Some(entity) = player_entity.0 {
        if let Ok(mut camera) = camera_query.get_single_mut() {
            if camera.target == Entity::PLACEHOLDER {
                camera.target = entity;
                println!("Camera target set to player entity: {:?}", entity);
            }
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    camera_query: Query<&ThirdPersonCamera>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut transform, mut velocity)) = player_query.get_single_mut() {
        let mut movement = Vec3::ZERO;
        
        // WASD movement
        if keyboard_input.pressed(KeyCode::W) {
            movement.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            movement.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            movement.x += 1.0;
        }
        
        // Normalize movement vector
        if movement.length() > 0.0 {
            movement = movement.normalize();
            
            // Get camera rotation to align movement with camera view
            let camera_rotation = if let Ok(camera) = camera_query.get_single() {
                camera.current_rotation
            } else {
                0.0
            };
            
            // Rotate movement based on camera rotation
            let cos_rot = camera_rotation.cos();
            let sin_rot = camera_rotation.sin();
            let rotated_movement = Vec3::new(
                movement.x * cos_rot - movement.z * sin_rot,
                0.0,
                movement.x * sin_rot + movement.z * cos_rot,
            );
            
            // Apply movement to velocity
            let target_velocity = rotated_movement * player.speed;
            velocity.linvel.x = target_velocity.x;
            velocity.linvel.z = target_velocity.z;
            
            // Update player rotation to face movement direction
            let target_rotation = Quat::from_rotation_arc(Vec3::Z, rotated_movement);
            transform.rotation = transform.rotation.slerp(target_rotation, player.rotation_speed * time.delta_seconds());
        } else {
            // Apply friction when not moving
            velocity.linvel.x *= 0.9;
            velocity.linvel.z *= 0.9;
        }
        
        // Jump
        if keyboard_input.just_pressed(KeyCode::Space) && player.on_ground {
            velocity.linvel.y = player.jump_force;
            player.on_ground = false;
        }
        
        // Sprint
        if keyboard_input.pressed(KeyCode::ShiftLeft) && movement.length() > 0.0 {
            velocity.linvel.x *= 1.5;
            velocity.linvel.z *= 1.5;
        }
    } else {
        println!("ERROR: No player found in movement system!");
    }
}

fn ground_detection(
    mut player_query: Query<(&mut Player, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((mut player, transform)) = player_query.get_single_mut() {
        let ray_origin = transform.translation;
        let ray_dir = Vec3::Y * -1.0;
        let max_distance = 1.1; // Slightly more than player height
        
        if let Some((_entity, toi)) = rapier_context.cast_ray(ray_origin, ray_dir, max_distance, true, QueryFilter::default()) {
            if toi < max_distance {
                player.on_ground = true;
            } else {
                player.on_ground = false;
            }
        } else {
            player.on_ground = false;
        }
    }
}

fn debug_player_state(
    player_query: Query<(&Player, &Transform, &Velocity)>,
    time: Res<Time>,
) {
    static mut LAST_LOG_TIME: f32 = 0.0;
    
    unsafe {
        if time.elapsed_seconds() - LAST_LOG_TIME > 2.0 {
            if let Ok((player, transform, velocity)) = player_query.get_single() {
                println!("=== PLAYER DEBUG ===");
                println!("Position: {:?}", transform.translation);
                println!("Velocity: {:?}", velocity.linvel);
                println!("On ground: {}", player.on_ground);
                println!("Speed: {}", player.speed);
                println!("Jump force: {}", player.jump_force);
                println!("===================");
            } else {
                println!("ERROR: No player found in debug system!");
            }
            LAST_LOG_TIME = time.elapsed_seconds();
        }
    }
} 