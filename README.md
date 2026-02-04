# Path Tracing Renderer

The project is a simple path tracing renderer implemented in `Rust` language. It demonstrates basic concepts of path tracing, including scene setup, camera configuration, and rendering.

## Features

- [x] Implemented CPU multithreading using `rayon`.
- [x] Supports directly rendering the images as `PNG` instead of `PPM` format.
- [x] Denoising and more realistic pixel color through Monte Carlo integration and weighted PDF.

## Future Work

- [x] PDF with light source directivity
- [ ] Add support for GPU acceleration using `wgpu`.
    - [ ] POD structures for GPU data transfer.
    - [ ] Shader implementation for path tracing on GPU.

## References

- [Ray Tracing in One Weekend - Book Series](https://raytracing.github.io/): A popular book that provides a comprehensive introduction to path tracing.