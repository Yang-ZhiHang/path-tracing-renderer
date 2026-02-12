# Path Tracing Renderer

The project is a simple path tracing renderer implemented in `Rust` language. It demonstrates basic concepts of path tracing, including scene setup, camera configuration, and rendering.

## Features

- [x] Implemented CPU multithreading using `rayon`.
- [x] Supports directly rendering the images as `PNG` instead of `PPM` format.
- [x] A unified BSDF-based scattering model (Microfacet BRDF/BTDF) for all materials, replacing separate scatter implementations. Supports Roughness, Metallic, IOR, and Transparency.
- [x] Able to set the shutting time for motion blur effects.

## Future Work

- [ ] Add texture system and constant medium for the BSDF-based(current) material system.
- [ ] Add support for GPU acceleration using `wgpu`.

## Attention

- In order to avoid numerical issues, the index of refraction and roughness values are clamped to safe ranges in `material.rs`.
- If you wanna make a hollow glass sphere, it's better to set the index of refraction of the inner sphere to reciprocal value (e.g., 1.0 / 1.5) than setting the radius to negative value.
- Currently, the renderer doesn't support to render the shape of light sources directly. You can only see their effects on other objects in the scene.

## References

- [Ray Tracing in One Weekend - Book Series](https://raytracing.github.io/): A popular book that provides a comprehensive introduction to path tracing.