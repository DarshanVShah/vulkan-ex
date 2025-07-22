use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod player;
mod terrain;
mod vulkan_renderer;

use camera::CameraPlugin;
use player::PlayerPlugin;
use terrain::TerrainPlugin;
use vulkan_renderer::VulkanRendererPlugin;

fn main() {
    env_logger::init();
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(VulkanRendererPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TerrainPlugin)
        .run();
}
