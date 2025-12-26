use indicatif::{ProgressBar, ProgressStyle};
use rand::random_range;
use simple_rpt::camera::Camera;
use simple_rpt::common::random;
use simple_rpt::config::load_config;
use simple_rpt::material::dieletrics::Dielectric;
use simple_rpt::material::lambertian::Lambertian;
use simple_rpt::material::metal::Metal;
use simple_rpt::math::{Color, Point3, Vec3, Vec3Ext};
use simple_rpt::object::Object;
use simple_rpt::renderer::Renderer;
use simple_rpt::scene::Scene;
use simple_rpt::shape::sphere::Sphere;

fn random_scene() -> Scene {
    let mut world = Scene::new();

    let ground = Object::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0))
        .material(Lambertian::new(Color::splat(0.5)));
    world.add(ground);

    const NUM: i32 = 11;
    for a in -NUM..NUM {
        for b in -NUM..NUM {
            let choose_mat = random();
            let center = Point3::new(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Object::new(Sphere::new(center, 0.2)).material(sphere_material));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Object::new(Sphere::new(center, 0.2)).material(sphere_material));
                } else {
                    // Glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Object::new(Sphere::new(center, 0.2)).material(sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Object::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0)).material(material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Object::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0)).material(material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Object::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0)).material(material3));

    world
}

fn main() {
    // Image
    let config = load_config("examples/config_sphere.toml").resolve();
    let aspect_ratio = config.image.aspect_ratio;
    let image_width = config.image.width;
    let image_height = config.image.height;
    let samples_per_pixel = config.samples_per_pixel;
    let depth = config.max_depth;
    print!(
        "Rendering image of size {}x{} with {} samples per pixel and max depth {}\n",
        image_width, image_height, samples_per_pixel, depth
    );

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let focal_length = 10.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        focal_length,
    );

    // Scene
    let mut scene = random_scene();
    scene.build_bvh();

    // Render
    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    let r = Renderer::new(cam, scene)
        .width(image_width)
        .height(image_height)
        .progress_bar(pb)
        .output_path(config.image.output_path);
    r.render(image_width, image_height, samples_per_pixel, depth);
}
