# MPDS

Molecular Physics Dynamics Software (MPDS) is a high-performance simulation and computation engine designed for general physics, molecular dynamics, and mathematical modeling. MPDS delivers real-time visualization and numerical solving for complex systems.

Currently under development, it will provide the following:

* **General Physics Engine:** Simulates classical mechanics, multi-body systems, fluid dynamics, and molecular structures.
* **Mathematical Solver:** High-performance matrix operations, differential equation integration, and coordinate transformations.
* **Fluid Dynamics**: Simulates fluids and gases for airflow testing.
* **CAD Support**: Able to import CAD models and simulate with them for structural engineering, thermal management, ...

## Architecture

* **Backend:** Written in rust, for memory safety, fast speeds and compile time safety.
* **Compute:** Written in GLSL/Vulkan for GPU-bound heavy math pipelines.
* **Frontend:** Written in Svelte for real-time visualization and user configuration.

## License

Distributed under the MIT License. See `LICENSE` for more information.
