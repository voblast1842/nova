# MPDS

Molecular Physics Dynamics Software (MPDS) **WILL BE** a high-performance simulation and computation engine designed for molecular dynamics modeling. MPDS **WILL** deliver real-time visualization and numerical solving for complex systems.

Currently under development, it **WILL** provide software for simulating:

- **Full-Stack Molecular Simulation:**
  - **Atomic Level** for individual atom tracking with standard CPK color models, charge distribution, and energy states.
  - **Dynamic Bonding Engine** for real-time bond breaking, electron sharing, and force field calculations.
  - **Macro-Molecular Scale** for simulation of complex multi-element compounds and 3D structures.
- **Quality of Life and Performance:**
  - **Dynamic Level Of Detail (LOD)** for performance boosts (Zooming out **WILL** condense atom clusters into averaged macro-particles).
  - **Scriptable workflows** for scientific automation and custom experiment setups.

## Project milestones
Active development tasks, upcoming features, and milestones **WILL BE** tracked publicly. (see above)

[] Molecular bond simulation
[] Individual atom simulation
[] Compound simulation
[] Dynamic level of detail
[] Scriptable workflows

## Physics & Mathematical Models

The underlying physics engine WILL rely on classical force fields and numerical integration techniques:

- **Non-bonded Interactions**: Lennard-Jones potential & Electrostatic forces (Coulomb's Law).
- **Bond Physics**: Harmonic bond/angle potentials or Morse potentials.
- **Integration**: Velocity Verlet / Leapfrog integration algorithms running on GPU compute pipelines.

## Architecture

* **Backend**: **WILL BE** written in Rust, for memory safety, fast speeds and compile time safety.
* **Compute** **WILL BE** written in GLSL/Vulkan (via wgpu/ash) for GPU-accelerated particle integration and real-time 3D shader rendering.
* **Frontend**: **WILL BE** built with Slint to provide a lightweight, zero-webview native desktop interface.
* **Scripting layer**: **WILL BE** exposed to Python via PyO3 bindings, allowing for headless batch processing, data export, and Jupyter Notebook integration.

## License

All of the code in this repository is released under Apache 2.0 / MIT / GPLv3 triple-license.
