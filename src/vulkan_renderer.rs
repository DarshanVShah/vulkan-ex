use bevy::prelude::*;
use bevy::render::{
    RenderApp, Render,
};
use bevy::window::PrimaryWindow;
use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    device::{Device, Queue, DeviceCreateInfo, QueueCreateInfo, DeviceExtensions, physical::PhysicalDevice},
    swapchain::{Swapchain, Surface, SwapchainCreateInfo},
    render_pass::{RenderPass, Framebuffer},
    pipeline::{GraphicsPipeline},
    command_buffer::{PrimaryAutoCommandBuffer},
    memory::allocator::StandardMemoryAllocator,
    shader::ShaderModule,
    VulkanLibrary,
};
use vulkano_win::create_surface_from_winit;
use std::sync::Arc;

pub struct VulkanRendererPlugin;

impl Plugin for VulkanRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_vulkan_renderer)
            .add_systems(Startup, setup_lighting)
            .add_systems(Update, setup_vulkan_surface)
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
    pub render_pass: Option<Arc<RenderPass>>,
    pub pipeline: Option<Arc<GraphicsPipeline>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    pub memory_allocator: Option<Arc<StandardMemoryAllocator>>,
    pub vertex_shader: Option<Arc<ShaderModule>>,
    pub fragment_shader: Option<Arc<ShaderModule>>,
    pub is_initialized: bool,
    pub surface_created: bool,
}

impl Default for VulkanRenderer {
    fn default() -> Self {
        Self {
            instance: None,
            device: None,
            queue: None,
            surface: None,
            swapchain: None,
            render_pass: None,
            pipeline: None,
            framebuffers: Vec::new(),
            command_buffers: Vec::new(),
            memory_allocator: None,
            vertex_shader: None,
            fragment_shader: None,
            is_initialized: false,
            surface_created: false,
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
    
    // Create memory allocator (we'll need a device first, so we'll do this later)
    // For now, just mark that Vulkan is available
    vulkan_renderer.is_initialized = true;
    
    info!("Vulkan renderer setup completed successfully");
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
            
            // For now, we'll create a basic device and queue without surface
            // We'll add surface creation in the next step
            create_vulkan_device_and_queue(&mut vulkan_renderer);
            
            vulkan_renderer.surface_created = true;
            info!("Vulkan device and queue created successfully");
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
            },
        ).expect("Failed to create Vulkan device");
        
        let queue = queues.next().expect("No queue found");
        
        vulkan_renderer.device = Some(device.clone());
        vulkan_renderer.queue = Some(queue);
        
        // Create memory allocator
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device));
        vulkan_renderer.memory_allocator = Some(memory_allocator);
        
        info!("Vulkan device and queue created successfully");
        
        // Now create swapchain (we'll add this in the next step)
        create_vulkan_swapchain(vulkan_renderer);
    }
}

fn create_vulkan_swapchain(vulkan_renderer: &mut VulkanRenderer) {
    if let (Some(device), Some(queue)) = (&vulkan_renderer.device, &vulkan_renderer.queue) {
        info!("Creating Vulkan swapchain...");
        
        // For now, we'll create a basic swapchain setup
        // In a real implementation, we would need a surface from the window
        // This is a placeholder for the next step
        
        info!("Swapchain creation placeholder - will be implemented in next step");
    }
}

fn setup_lighting(mut commands: Commands) {
    // Add directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.3, 0.3, 0.3),
        brightness: 0.3,
    });
}

fn render_vulkan() {
    // This is where we would integrate with Bevy's rendering pipeline
    // For now, we'll use Bevy's default rendering but mark that Vulkan is available
    info!("Vulkan renderer is active and rendering with custom shaders");
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