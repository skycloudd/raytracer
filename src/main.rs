use camera::Camera;
use glam::Vec3;

mod camera;
mod colour;
mod hit;
mod interval;
mod materials;
mod objects;
mod ray;
mod scenes;

const WIDTH: u32 = 800;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const VERTICAL_FOV: f32 = 60.0;

const LOOK_FROM: Vec3 = Vec3::new(0.0, 0.0, 2.0);
const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);

const SAMPLES_PER_PIXEL_CONFIG: u32 = 3;
const SAMPLES_PER_PIXEL: u32 = SAMPLES_PER_PIXEL_CONFIG * SAMPLES_PER_PIXEL_CONFIG;
const MAX_DEPTH: u16 = 50;

const DEFOCUS_ANGLE: f32 = 0.0;
const FOCUS_DISTANCE: f32 = 1.5;

fn main() {
    let world = scenes::three_spheres();
    // let world = scenes::rainbow_spheres();

    let mut camera = Camera::new();

    camera.render(&world);
}
