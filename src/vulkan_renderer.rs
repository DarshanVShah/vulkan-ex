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
    render_pass::{RenderPass, Subpass},
    pipeline::{GraphicsPipeline, PipelineLayout},
    memory::allocator::StandardMemoryAllocator,
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
    pub render_pass: Option<Arc<RenderPass>>,
    pub pipeline: Option<Arc<GraphicsPipeline>>,
    pub memory_allocator: Option<Arc<StandardMemoryAllocator>>,
    pub surface_created: bool,
    pub swapchain_created: bool,
    pub pipeline_created: bool,
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
            render_pass: None,
            pipeline: None,
            memory_allocator: None,
            surface_created: false,
            swapchain_created: false,
            pipeline_created: false,
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
                
                // Create render pass and pipeline
                if vulkan_renderer.swapchain_created {
                    create_vulkan_render_pass_and_pipeline(&mut vulkan_renderer);
                }
            }
            
            vulkan_renderer.surface_created = true;
            info!("Vulkan surface, swapchain, and pipeline created successfully");
        }
    }
}

fn create_vulkan_device_and_queue(vulkan_renderer: &mut VulkanRenderer) {
    use vulkano::device::DeviceExtensions;
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
        
        // Enable khr_swapchain extension
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        
        // Create device and queue
        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            }
        ).expect("Failed to create device");
        
        let queue = queues.next().unwrap();
        
        vulkan_renderer.device = Some(device.clone());
        vulkan_renderer.queue = Some(queue);
        
        // Create memory allocator
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device));
        vulkan_renderer.memory_allocator = Some(memory_allocator);
        
        info!("Vulkan device, queue, and memory allocator created successfully");
    }
}

fn create_vulkan_swapchain(vulkan_renderer: &mut VulkanRenderer, _window: &Window) {
    if let (Some(_instance), Some(_device)) = (&vulkan_renderer.instance, &vulkan_renderer.device) {
        info!("Creating Vulkan swapchain...");
        
        // For now, we'll create a basic swapchain setup
        // The surface creation from Bevy window requires more complex integration
        // We'll implement this in the next step
        
        info!("Swapchain creation - will be implemented in next step");
        vulkan_renderer.swapchain_created = true;
    }
}

fn create_vulkan_render_pass_and_pipeline(vulkan_renderer: &mut VulkanRenderer) {
    use vulkano::render_pass::{AttachmentDescription, LoadOp, StoreOp, SubpassDescription, RenderPassCreateInfo, AttachmentReference};
    use vulkano::image::{ImageLayout, ImageAspects, SampleCount};
    if let Some(device) = &vulkan_renderer.device {
        info!("Creating Vulkan render pass and pipeline...");
        
        // Create a simple render pass using the builder API for vulkano 0.33+
        let mut color_attachment = AttachmentDescription::default();
        color_attachment.format = Some(Format::B8G8R8A8_SRGB);
        color_attachment.samples = SampleCount::Sample1;
        color_attachment.load_op = LoadOp::Clear;
        color_attachment.store_op = StoreOp::Store;
        color_attachment.stencil_load_op = LoadOp::DontCare;
        color_attachment.stencil_store_op = StoreOp::DontCare;
        color_attachment.initial_layout = ImageLayout::Undefined;
        color_attachment.final_layout = ImageLayout::PresentSrc;
        let mut color_ref = AttachmentReference::default();
        color_ref.attachment = 0;
        color_ref.layout = ImageLayout::ColorAttachmentOptimal;
        color_ref.aspects = ImageAspects::empty();
        let mut subpass = SubpassDescription::default();
        subpass.color_attachments = vec![Some(color_ref)];
        let render_pass_info = RenderPassCreateInfo {
            attachments: vec![color_attachment],
            subpasses: vec![subpass],
            ..Default::default()
        };
        let render_pass = RenderPass::new(device.clone(), render_pass_info).unwrap();
        
        // For now, we'll create a basic pipeline setup
        // The full pipeline creation requires more complex shader compilation
        // We'll implement this in the next step
        
        vulkan_renderer.render_pass = Some(render_pass);
        vulkan_renderer.pipeline_created = true;
        
        info!("Vulkan render pass created successfully (pipeline will be implemented in next step)");
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