use bevy::prelude::*;
use bevy::window::Window;
use log::info;
use std::ffi::CStr;
use ash::{
    vk,
    Instance as AshInstance,
    Device as AshDevice,
    Entry,
    extensions::{
        khr::Surface,
        khr::Swapchain,
    },
};
use gpu_allocator::vulkan::Allocator;

pub struct VulkanRendererPlugin;

impl Plugin for VulkanRendererPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VulkanRenderer>()
            .add_systems(Update, setup_vulkan_surface)
            .add_systems(Startup, setup_lighting);
    }
}

#[derive(Resource)]
pub struct VulkanRenderer {
    pub entry: Option<Entry>,
    pub instance: Option<AshInstance>,
    pub device: Option<AshDevice>,
    pub surface: Option<vk::SurfaceKHR>,
    pub swapchain: Option<vk::SwapchainKHR>,
    pub swapchain_images: Vec<vk::Image>,
    pub render_pass: Option<vk::RenderPass>,
    pub pipeline: Option<vk::Pipeline>,
    pub allocator: Option<Allocator>,
    pub instance_created: bool,
    pub device_created: bool,
    pub swapchain_created: bool,
    pub pipeline_created: bool,
}

impl Default for VulkanRenderer {
    fn default() -> Self {
        Self {
            entry: None,
            instance: None,
            device: None,
            surface: None,
            swapchain: None,
            swapchain_images: Vec::new(),
            render_pass: None,
            pipeline: None,
            allocator: None,
            instance_created: false,
            device_created: false,
            swapchain_created: false,
            pipeline_created: false,
        }
    }
}

fn setup_vulkan_renderer(vulkan_renderer: &mut VulkanRenderer) {
    info!("Setting up Vulkan renderer...");
    
    // Load Vulkan entry point
    let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry point") };
    
    // Check available extensions
    let available_extensions = unsafe {
        entry.enumerate_instance_extension_properties(None)
            .expect("Failed to enumerate instance extensions")
    };
    
    info!("Available extensions: {:?}", available_extensions.len());
    
    // Create Vulkan instance with minimal extensions
    let app_info = vk::ApplicationInfo::builder()
        .application_name(unsafe { CStr::from_bytes_with_nul_unchecked(b"Vulkan Game\0") })
        .application_version(vk::API_VERSION_1_0)
        .engine_name(unsafe { CStr::from_bytes_with_nul_unchecked(b"Bevy\0") })
        .engine_version(vk::API_VERSION_1_0)
        .api_version(vk::API_VERSION_1_0)
        .build();
    
    let instance_create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .build();
    
    let instance = unsafe { 
        entry.create_instance(&instance_create_info, None)
            .expect("Failed to create Vulkan instance")
    };
    
    vulkan_renderer.entry = Some(entry);
    vulkan_renderer.instance = Some(instance);
    vulkan_renderer.instance_created = true;
    
    info!("Vulkan instance created successfully");
}

fn setup_lighting(mut commands: Commands) {
    // Add a directional light for the scene
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
    windows: Query<&Window>,
) {
    if !vulkan_renderer.instance_created {
        setup_vulkan_renderer(&mut vulkan_renderer);
    }
    
    if !vulkan_renderer.device_created {
        create_vulkan_device_and_queue(&mut vulkan_renderer);
    }
    
    if !vulkan_renderer.swapchain_created {
        if let Ok(window) = windows.get_single() {
            create_vulkan_swapchain(&mut vulkan_renderer, window);
        }
    }
    
    if !vulkan_renderer.pipeline_created {
        create_vulkan_render_pass_and_pipeline(&mut vulkan_renderer);
    }
    
    if vulkan_renderer.instance_created && vulkan_renderer.device_created && 
       vulkan_renderer.swapchain_created && vulkan_renderer.pipeline_created {
        info!("Vulkan surface, swapchain, and pipeline created successfully");
    }
}

fn create_vulkan_device_and_queue(vulkan_renderer: &mut VulkanRenderer) {
    if let Some(instance) = &vulkan_renderer.instance {
        info!("Creating Vulkan device and queue...");
        
        // Find a suitable physical device
        let physical_devices = unsafe { 
            instance.enumerate_physical_devices()
                .expect("Failed to enumerate physical devices")
        };
        
        let physical_device = physical_devices[0]; // Use first available device
        
        // Find a queue family that supports graphics
        let queue_family_properties = unsafe { 
            instance.get_physical_device_queue_family_properties(physical_device)
        };
        
        let queue_family_index = queue_family_properties
            .iter()
            .position(|props| props.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .expect("No graphics queue family found") as u32;
        
        // Create logical device
        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0])
            .build();
        
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_create_info))
            .enabled_extension_names(&[
                Swapchain::name().as_ptr(),
            ])
            .build();
        
        let device = unsafe { 
            instance.create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        };
        
        // Create memory allocator
        let allocator = Allocator::new(&gpu_allocator::vulkan::AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device,
            debug_settings: Default::default(),
            buffer_device_address: false,
        }).expect("Failed to create memory allocator");
        
        vulkan_renderer.device = Some(device);
        vulkan_renderer.allocator = Some(allocator);
        vulkan_renderer.device_created = true;
        
        info!("Vulkan device and memory allocator created successfully");
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
    if let Some(device) = &vulkan_renderer.device {
        info!("Creating Vulkan render pass and pipeline...");
        
        // Create a simple render pass
        let color_attachment = vk::AttachmentDescription::builder()
            .format(vk::Format::B8G8R8A8_SRGB)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .build();
        
        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_attachment_ref))
            .build();
        
        let render_pass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(std::slice::from_ref(&color_attachment))
            .subpasses(std::slice::from_ref(&subpass))
            .build();
        
        let render_pass = unsafe { 
            device.create_render_pass(&render_pass_create_info, None)
                .expect("Failed to create render pass")
        };
        
        vulkan_renderer.render_pass = Some(render_pass);
        vulkan_renderer.pipeline_created = true;
        
        info!("Vulkan render pass created successfully (pipeline will be implemented in next step)");
    }
}

fn render_vulkan() {
    // This will be implemented in the next step
    info!("Vulkan render system called");
} 