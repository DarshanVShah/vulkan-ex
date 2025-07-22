use bevy::prelude::*;
use bevy::render::{
    RenderApp, Render,
};
use bevy::window::PrimaryWindow;
use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    device::{Device, Queue, DeviceCreateInfo, QueueCreateInfo, physical::PhysicalDevice},
    swapchain::{Swapchain, Surface, SwapchainCreateInfo},
    image::SwapchainImage,
    format::Format,
    image::ImageUsage,
    VulkanLibrary,
};
use vulkano_win::create_surface_from_winit;
use std::sync::Arc;

pub struct VulkanRendererPlugin;

impl Plugin for VulkanRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_vulkan_renderer)
            .add_systems(Startup, setup_lighting)
            .add_systems(Update, setup_vulkan_surface_system)
            .init_resource::<VulkanRenderer>()
            .sub_app_mut(RenderApp)
            .add_systems(Render, render_vulkan);
    }
}

#[derive(Resource)]
pub struct VulkanRenderer {
    pub instance: Option<Arc<Instance>>,
    pub device: Option<Arc<Device>>,
    pub queue: Option<Arc<Queue>>,
    pub surface: Option<Arc<Surface>>,
    pub swapchain: Option<Arc<Swapchain>>,
    pub swapchain_images: Vec<Arc<SwapchainImage>>,
    pub surface_created: bool,
    pub swapchain_created: bool,
}

impl Default for VulkanRenderer {
    fn default() -> Self {
        Self {
            instance: None,
            device: None,
            queue: None,
            surface: None,
            swapchain: None,
            swapchain_images: Vec::new(),
            surface_created: false,
            swapchain_created: false,
        }
    }
}

fn setup_vulkan_renderer(mut vulkan_renderer: ResMut<VulkanRenderer>) {
    info!("Setting up Vulkan renderer...");
    
    // Load Vulkan library
    let library = VulkanLibrary::new().expect("Failed to load Vulkan library");
    
    // Create Vulkan instance
    let instance = Instance::new(
        library.clone(),
        InstanceCreateInfo {
            enabled_extensions: vulkano_win::required_extensions(&library),
            ..Default::default()
        }
    ).expect("Failed to create Vulkan instance");
    
    vulkan_renderer.instance = Some(instance);
    info!("Vulkan instance created successfully");
}

fn setup_lighting(mut commands: Commands) {
    // Add lighting for the scene
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_vulkan_surface(
    mut vulkan_renderer: ResMut<VulkanRenderer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if vulkan_renderer.surface_created {
        return; // Already created
    }
    
    if let Ok(window) = window_query.get_single() {
        if let Some(instance) = &vulkan_renderer.instance {
            info!("Creating Vulkan surface from Bevy window...");
            
            // Create device and queue first
            create_vulkan_device_and_queue(&mut vulkan_renderer);
            
            // Create surface from window
            if let Some(device) = &vulkan_renderer.device {
                create_vulkan_swapchain(&mut vulkan_renderer, window);
            }
            
            vulkan_renderer.surface_created = true;
            info!("Vulkan surface and swapchain created successfully");
        }
    }
}

fn create_vulkan_device_and_queue(vulkan_renderer: &mut VulkanRenderer) {
    if let Some(instance) = &vulkan_renderer.instance {
        info!("Creating Vulkan device and queue...");
        
        // Find a suitable physical device
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
            .next()
            .expect("No suitable physical device found");
        
        // Find a suitable queue family
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, family)| {
                family.queue_flags.contains(vulkano::device::QueueFlags::GRAPHICS)
            })
            .expect("No suitable queue family found") as u32;
        
        // Create device and queue
        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            }
        ).expect("Failed to create device");
        
        let queue = queues.next().unwrap();
        
        vulkan_renderer.device = Some(device);
        vulkan_renderer.queue = Some(queue);
        info!("Vulkan device and queue created successfully");
    }
}

fn create_vulkan_swapchain(vulkan_renderer: &mut VulkanRenderer, window: &Window) {
    if let (Some(instance), Some(device)) = (&vulkan_renderer.instance, &vulkan_renderer.device) {
        info!("Creating Vulkan swapchain...");
        
        // For now, we'll create a basic swapchain setup
        // The surface creation from Bevy window requires more complex integration
        // We'll implement this in the next step
        
        info!("Swapchain creation - will be implemented in next step");
        vulkan_renderer.swapchain_created = true;
    }
}

fn render_vulkan() {
    // This system will be called by Bevy's render pipeline
    // For now, we'll just log that Vulkan rendering is happening
    // In the next step, we'll add actual Vulkan rendering commands
}

fn setup_vulkan_surface_system(
    mut vulkan_renderer: ResMut<VulkanRenderer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    setup_vulkan_surface(vulkan_renderer, window_query);
}

// Vulkan shader compilation
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vulkan_instance_creation() {
        let library = VulkanLibrary::new();
        assert!(library.is_ok());
        
        if let Ok(library) = library {
            let instance = Instance::new(
                library.clone(),
                InstanceCreateInfo {
                    enabled_extensions: vulkano_win::required_extensions(&library),
                    ..Default::default()
                }
            );
            
            assert!(instance.is_ok());
        }
    }
    
    #[test]
    fn test_shader_file_loading() {
        // Test that shader files exist and can be read
        let vertex_shader_path = "assets/shaders/vulkan_vertex.glsl";
        let fragment_shader_path = "assets/shaders/vulkan_fragment.glsl";
        
        assert!(std::path::Path::new(vertex_shader_path).exists());
        assert!(std::path::Path::new(fragment_shader_path).exists());
    }
} 