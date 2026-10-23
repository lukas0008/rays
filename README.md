# rays

A small toy CPU ray tracer written in Rust. Scenes are rendered by casting one ray per pixel against triangle meshes, with the resulting framebuffer displayed in a `winit` window using `wgpu`.

The project includes:

- Triangle and plane intersection
- Quaternion-based camera and model rotation
- Basic Wavefront OBJ loader
- Parallel rendering with Rayon
- Progressive pixel updates

## Requirements

- Rust nightly (`portable_simd` is currently used)
- A graphics environment supported by `winit` and `wgpu`

Install nightly with [rustup](https://rustup.rs/):

```sh
rustup toolchain install nightly
```

## Running

```sh
cargo +nightly run --release
```

The default scene contains two test triangles and the monkey mesh from `src/monki.obj`.

## Controls

| Key | Action |
| --- | --- |
| `W` / `S` | Move camera along the Z axis |
| `A` / `D` | Move camera along the X axis |
| `Space` / `Left Ctrl` | Move camera up/down |
| Arrow keys | Rotate the camera |
| `Enter` | Render another frame |

Each accepted input starts a new render and advances the monkey model's rotation slightly.

## Notes
This readme was ai assisted

