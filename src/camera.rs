use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow)
            .add_systems(Update, camera_rotation)
            .add_systems(Update, camera_zoom)
            .add_systems(Update, debug_camera_state);
    }
}

#[derive(Component)]
pub struct ThirdPersonCamera {
    pub target: Entity,
    pub distance: f32,
    pub height: f32,
    pub smoothness: f32,
    pub rotation_speed: f32,
    pub current_rotation: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
}

fn setup_camera(mut commands: Commands) {
    println!("=== SETTING UP CAMERA ===");
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            target: Entity::PLACEHOLDER,
            distance: 8.0,
            height: 3.0,
            smoothness: 5.0,
            rotation_speed: 2.0,
            current_rotation: 0.0,
            min_distance: 3.0,
            max_distance: 15.0,
            zoom_speed: 1.0,
        },
    ));
    println!("Camera spawned with placeholder target");
}

fn camera_follow(
    mut camera_query: Query<(&mut Transform, &mut ThirdPersonCamera)>,
    player_query: Query<&Transform, (With<Player>, Without<ThirdPersonCamera>)>,
    time: Res<Time>,
) {
    if let Ok((mut camera_transform, mut camera)) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get(camera.target) {
            let target_pos = player_transform.translation;
            let target_pos_with_height = target_pos + Vec3::Y * camera.height;
            
            // Calculate camera position based on rotation
            let rotation_rad = camera.current_rotation;
            let camera_offset = Vec3::new(
                rotation_rad.sin() * camera.distance,
                0.0,
                rotation_rad.cos() * camera.distance,
            );
            let desired_pos = target_pos_with_height + camera_offset;
            
            // Smoothly interpolate camera position
            let current_pos = camera_transform.translation;
            let new_pos = current_pos.lerp(desired_pos, camera.smoothness * time.delta_seconds());
            
            camera_transform.translation = new_pos;
            camera_transform.look_at(target_pos_with_height, Vec3::Y);
        } else {
            println!("Camera: Player not found, target entity: {:?}", camera.target);
        }
    } else {
        println!("Camera: No camera found");
    }
}

fn camera_rotation(
    mut camera_query: Query<&mut ThirdPersonCamera>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        // Handle mouse rotation when right mouse button is held
        if mouse_input.pressed(MouseButton::Right) {
            for ev in mouse_motion.read() {
                let rotation_delta = ev.delta.x * camera.rotation_speed * time.delta_seconds() * 0.01;
                camera.current_rotation -= rotation_delta;
                println!("Camera rotation: {} (delta: {})", camera.current_rotation, rotation_delta);
            }
        }
    }
}

fn camera_zoom(
    mut camera_query: Query<&mut ThirdPersonCamera>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        for ev in scroll_evr.read() {
            let zoom_delta = ev.y * camera.zoom_speed * 0.1;
            let old_distance = camera.distance;
            camera.distance = (camera.distance - zoom_delta)
                .clamp(camera.min_distance, camera.max_distance);
            println!("Camera zoom: {} -> {} (delta: {})", old_distance, camera.distance, zoom_delta);
        }
    }
}

fn debug_camera_state(
    camera_query: Query<&ThirdPersonCamera>,
    time: Res<Time>,
) {
    static mut LAST_LOG_TIME: f32 = 0.0;
    
    unsafe {
        if time.elapsed_seconds() - LAST_LOG_TIME > 3.0 {
            if let Ok(camera) = camera_query.get_single() {
                println!("=== CAMERA DEBUG ===");
                println!("Target entity: {:?}", camera.target);
                println!("Distance: {}", camera.distance);
                println!("Height: {}", camera.height);
                println!("Current rotation: {}", camera.current_rotation);
                println!("Rotation speed: {}", camera.rotation_speed);
                println!("===================");
            } else {
                println!("ERROR: No camera found in debug system!");
            }
            LAST_LOG_TIME = time.elapsed_seconds();
        }
    }
} 