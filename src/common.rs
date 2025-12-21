use rand::Rng;

pub fn random() -> f32 {
    rand::rng().random()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}
