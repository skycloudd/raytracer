pub type Range = std::ops::Range<f32>;

pub trait Interval {
    fn clamp(&self, value: f32) -> f32;

    fn min(&self) -> f32;

    fn max(&self) -> f32;
}

impl Interval for Range {
    fn clamp(&self, value: f32) -> f32 {
        if value < self.min() {
            self.min()
        } else if value > self.max() {
            self.max()
        } else {
            value
        }
    }

    fn min(&self) -> f32 {
        self.start
    }

    fn max(&self) -> f32 {
        self.end
    }
}
