# SIM - 3D Physics Simulation

A 3D physics simulation project built with Bevy game engine, supporting WebAssembly for browser deployment.

[中文版 README](README.zh.md)

## Demo

Try the online demo: [https://leido.github.io/sim/](https://leido.github.io/sim/)

## Tech Stack

- **Rust** - Systems programming language
- **Bevy 0.17.3** - Data-driven game engine
- **Avian3D 0.4.1** - 3D physics engine
- **bevy_egui** - ImGui-style UI framework
- **Trunk** - WASM build tool

## Features

- 3D physics simulation
- Car dynamics simulation
- Real-time UI panels
- Camera control system
- Sound effects system
- WebAssembly support

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)
- [Trunk](https://trunkrs.dev/) - WASM build tool

### Install Trunk

```bash
cargo install trunk
```

### Run Locally (Native)

```bash
cargo run
```

### Run Locally (WASM Mode)

```bash
trunk serve --open
```

Visit http://127.0.0.1:8080 to view the application.


## Development

### Project Structure

```
sim/
├── src/
│   ├── main.rs           # Application entry point
│   ├── camera.rs         # Camera control
│   ├── car_dynamics.rs   # Car dynamics
│   ├── init.rs           # Initialization
│   ├── input.rs          # Input handling
│   ├── panel.rs          # UI panels
│   ├── sound.rs          # Sound system
│   └── utils.rs          # Utility functions
├── assets/               # Asset files
├── index.html            # HTML entry point
├── index.scss            # Stylesheet
├── Cargo.toml            # Rust dependencies
└── Trunk.toml            # Trunk configuration
```

## License

[MIT](LICENSE)
