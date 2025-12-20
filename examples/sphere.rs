use std::path::PathBuf;

use indicatif::{ProgressBar, ProgressStyle};
use simple_rpt::camera::Camera;
use simple_rpt::config::load_config;
use simple_rpt::renderer::Renderer;
use simple_rpt::shape::{Scene, sphere::Sphere};

fn main() {
    // Image
    let config = load_config("config.toml");
    let aspect_ratio = config.aspect_ratio();
    let image_width = config.image_width();
    let image_height = config.image_height();

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio as f32 * viewport_height;
    let cm = Camera::new([0.0, 0.0, 0.0], 1.0, viewport_height, viewport_width);

    // World
    let sp1 = Sphere::new([0.8, 0.0, -1.0], 0.3);
    let sp2 = Sphere::new([-0.5, 0.0, -1.0], 0.2);
    let mut scene = Scene::new();
    scene.add(sp1);
    scene.add(sp2);

    // Render
    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );

    let r = Renderer::new(cm, scene, pb, PathBuf::from(config.output_path()));
    r.render(image_width, image_height);
}
