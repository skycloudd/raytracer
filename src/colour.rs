use glam::Vec3;

pub type Colour = Vec3;

pub fn rgb(r: u8, g: u8, b: u8) -> Colour {
    Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
