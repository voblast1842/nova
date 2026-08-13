# MPDS

Molecular Physics Dynamics Software (MPDS) **WILL BE** a high-performance simulation and computation engine designed for molecular dynamics modeling. MPDS **WILL** deliver real-time visualization and numerical solving for complex systems.

Currently under development, it **WILL** provide software for simulating:

- **Molecular Bonds** and electron sharing, atomic charge, and energy changes.
- **Individiual atom simulation** for simulating complex bonds.
- **Compound simulation** for simulating advanced compounds.
- **LOD** for performance boosts. (Zooming out condenses atoms into an average)

// ```mermaid
// flowchart LR
//   Browser["WSI Browser (Svelte)"] -->|HTTPS| Meta["meta (HTTP API)"]
//   Browser -->|WebSocket| Frusta["frusta (WS streaming)"]
//   Frusta -->|gRPC| Storage["storage (tile store)"]
//   Meta -->|Postgres| PG["PostgreSQL"]
//   Frusta -->|Redis| R["Redis"]
//   IAM["iam (HTTP API)"] -->|Keycloak| KC["Keycloak"]
//   Meta -->|JWT validation| KC
//   Compiler["compiler (JetStream worker)"] -->|NATS JetStream| NATS["NATS"]
//   Storage -->|cache-miss events| NATS
//   Compiler -->|S3-compatible| S3["Cold Storage (S3-compatible)"]
// ```

## Physics & Mathematical Models

- Lennard-Jones potential & Electrostatic forces.
- Harmonic bond/angle potentials or Morse potentials.

## Architecture

* **Backend:** Written in rust, for memory safety, fast speeds and compile time safety.
* **Compute:** **WILL BE** written in GLSL/Vulkan for GPU-bound heavy math pipelines.
* **Frontend:** **WILL BE** written in Svelte for real-time visualization and user configuration.

## License

Distributed under the MIT License. See `LICENSE` for more information.
