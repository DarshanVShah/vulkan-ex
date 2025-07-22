# Vulkan Game Example

A third-person 3D game built with Rust, Bevy, and Vulkan rendering.

## Features

- **Third-person character controller** with smooth movement
- **Mouse-controlled camera** with rotation and zoom
- **Physics-based movement** with jumping and ground detection
- **Floating island terrain** with trees, rocks, and platforms
- **Vulkan rendering** backend for high performance
- **Smooth camera following** with customizable settings

## Controls

### Player Movement
- **W/A/S/D** - Move forward/left/backward/right
- **Space** - Jump (only when on ground)
- **Left Shift** - Sprint (hold while moving)

### Camera Controls
- **Right Mouse Button** - Hold and drag to rotate camera around player
- **Mouse Wheel** - Zoom in/out
- Camera automatically follows the player smoothly

### Game Features
- **Physics-based movement** with realistic momentum and friction
- **Ground detection** for proper jumping mechanics
- **Camera-relative movement** - WASD always moves relative to camera view
- **Smooth player rotation** - Character faces movement direction
- **Floating platforms** - Explore the environment by jumping between platforms

## Technical Details

- **Engine**: Bevy 0.12
- **Physics**: Bevy Rapier3D
- **Rendering**: Vulkan (via WGPU)
- **Language**: Rust

## Building and Running

```bash
cargo run
```

## Game World

The game features a floating island with:
- Main platform with grass surface
- Decorative trees and rocks
- Multiple floating platforms at different heights
- Physics-based collision detection
- Dynamic lighting and shadows

Enjoy exploring the floating world! 