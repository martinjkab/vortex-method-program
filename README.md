# Vortex-Based Fluid Simulation (Örvényalapú folyadékszimuláció)

A real-time 3D fluid simulation engine built in Rust using OpenGL Compute Shaders. This project was developed as a Bachelor's Thesis at the Budapest University of Technology and Economics (BME).

**Author:** Jakab Martin  
**Supervisor:** Dr. Szécsi László  
**Thesis Link:** [Diplomaterv Portál](https://diplomaterv.vik.bme.hu/hu/Theses/Orvenyalapu-folyadekszimulacio)

## Abstract

Modeling and understanding physical phenomena is a key area of computer graphics and engineering. Fluid simulation has seen many approaches, one of which is the vortex-based method. The advantage of this method is its ability to model complex flow phenomena with relatively low computational requirements, enabling real-time simulations.

This project implements a vortex particle method that handles:

1.  **Fluid-Boundary Interaction:** Two methods were introduced to handle the interaction between the fluid and solid objects.
2.  **Remeshing/Particle Lifecycle:** A system to maintain a uniform distribution of fluid particles using a lifecycle model known from particle systems.
3.  **Real-time Visualization:** High-performance rendering of the simulation results.

## Features

- **Vortex Particle Method:** Efficient simulation of fluid dynamics using vorticity.
- **GPU Acceleration:** Heavy physics calculations are offloaded to the GPU using OpenGL Compute Shaders (`.comp` files).
- **Rust & OpenGL:** Written in Rust for safety and performance, leveraging the `glfw` crate for windowing and raw OpenGL bindings for rendering.
- **Interactive Camera:** Navigate the 3D scene to view the simulation from any angle.

## Prerequisites

- **Rust Toolchain:** Install from [rustup.rs](https://rustup.rs/).
- **OpenGL 4.3+:** Ensure your graphics drivers are up to date (Compute Shaders require OpenGL 4.3 or higher).

## Installation & Running

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/vortex-method-program.git
    cd vortex-method-program
    ```

2.  **Build and Run:**
    It is highly recommended to run in **release mode** for smooth simulation performance:
    ```bash
    cargo run --release
    ```

## Controls

The simulation typically features a free-flying camera:

- **W / A / S / D:** Move Camera (Forward/Left/Backward/Right)
- **Mouse:** Orientation/Look

## Technical Architecture

The project is structured into several key modules:

- `src/objects`: Handles simulation entities like `Particles`, `ActiveVorticies`, and boundary meshes.
- `src/structures`: Defines core data structures like `Vortex`, `Particle`, and `CubeGeometry`.
- `resources/shaders`: Contains the GLSL shaders.
  - `*.comp`: Compute shaders for physics updates (vortex interaction, advection, etc.).
  - `*.vert` / `*.frag`: Shaders for rendering the particles and meshes.

## License & Copyright

Copyright © BME Faculty of Electrical Engineering and Informatics.  
Refer to the [Thesis Page](https://diplomaterv.vik.bme.hu/hu/Theses/Orvenyalapu-folyadekszimulacio) for more details.
