use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize)]
pub struct Config {
    aspect_width: u32,
    aspect_height: u32,
    image_width: u32,
    output_path: String,
    
    pub samples_per_pixel: u32,
}

impl Config {
    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_width as f64 / self.aspect_height as f64
    }

    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    pub fn image_height(&self) -> u32 {
        (self.image_width as f64 / self.aspect_ratio()) as u32
    }

    pub fn output_path(&self) -> &Path {
        Path::new(&self.output_path)
    }
}

pub fn load_config(path_str: &str) -> Config {
    let path = Path::new(path_str);
    let content = fs::read_to_string(&path).expect("The config.toml is not exist.");
    toml::from_str(&content).unwrap()
}
