# Fractal Experiment

## January 2025

Originally created by Ivan Avdeev (w23) in w23/iced-fragment-shader-widget-example.
I have adapted it to newest versions of needed libraries.

The plan currently is to add additional fractal experiments, and eventually investigate GPU-accelerated arbitrary zoom fractal generation.

## Usage

This repository follows standard Rust processes.

- To run the project, use `cargo run` or `cargo run --release`.
- To build the project, use `cargo build` or `cargo build --release`.
- To run built-in tests, use `cargo test`.
- If you're having issues, make sure you're using the right version of Rust. This repository was developed with Rust 2024 stable.

## Dependencies

- Rust (2024)
- Iced (0.13)
  - GUI/Windowing library. Uses winit and wgpu backends.
- Glam (0.30)
  - Linear algebra library.
- Bytemuck (1.21)
  - Tools for dealing with repr(C) and casting between plain types.
  - Needed when passing data to shader.

## Diagram

```mermaid
graph LR
    A[Widget]
    C[Program]
    D[Primitive]
    E[Pipeline]
    F[Shader]

    A --> |Events| C
    C --> |Controls| D
    D --> |Uniforms| E
    E --> |Buffer| F
    E --> |Render Pass| F
```

## Planned Experiments

### Arbitrary Precision Fractal Generation

- Generate fractals with arbitrary precision, allowing for zooming in and out to any level.
  - Previous experiments of mine were limited by f32 precision, which limited how far you could zoom into them, or by slow performance of arbitrary precision math libraries.
  - The plan is to use arbitrary precision for specific stable values in the viewport, and then use f32 in the shader to calculate deltas between those values. This will allow us to generate fractals with higher resolution than before, while still maintaining reasonable performance.

### Julia Set Fractal Exploration

- Once it is possible to zoom beyond traditional f32 limitations, we can explore beyond the Mandelbrot and look into Julia sets further.
  - There is not much technical difference between exploring the Mandelbrot set and Julia sets, but there are some interesting differences in how they behave under different conditions.

