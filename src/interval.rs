#[derive(Clone, Copy)]
pub struct Interval {
    /// The minimal value of a interval.
    pub min: f32,

    /// The maximal value of a interval.
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn expand(&mut self, delta: f32) {
        self.min = self.min - delta;
        self.max = self.max + delta;
    }

    pub fn contains(&self, val: f32) -> bool {
        self.min <= val && val <= self.max
    }
}
