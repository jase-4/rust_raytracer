# Rust Raytracer

A parallel CPU-based raytracer written in Rust featuring materials (Diffuse,Metal,Glass) and OBJ model loading.
## Features

- **Parallel rendering** using Rayon for fast multi-core performance
- **Materials**: Lambertian (diffuse), Metal (reflective), Dielectric (glass)
- **OBJ file loading** for rendering 3D models
- **Multi-sampling** anti-aliasing

<img width="1200" height="675" alt="output5" src="https://github.com/user-attachments/assets/66441239-7336-4d32-b6ac-f2c279b54b66" />
<img width="1200" height="675" alt="output2" src="https://github.com/user-attachments/assets/25610f28-c2d3-469c-81a6-99cb6b3aae85" />
<img width="1200" height="675" alt="output7" src="https://github.com/user-attachments/assets/197b05c2-a671-4f6c-9c10-a0cac6b0def0" />



## Requirements

- Rust (latest stable)
- Cargo

## Installation

```bash
git clone https://github.com/jase-4/rust_raytracer.git
cd rust_raytracer
cargo build
```

## Usage

Run the raytracer:

```bash
cargo run
```

The output will be saved as `img/output.png`.

## Configuration

Edit `main.rs` to adjust render settings:

```rust
img_width: 1200,           // Image width in pixels
samples_per_pixel: 100,    // Higher = less noise, slower render
max_depth: 50,             // Maximum ray bounces
vfov: 45.0,                // Vertical field of view (degrees)
```
