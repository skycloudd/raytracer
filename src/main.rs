use camera::Camera;
use colour::rgb;
use glam::Vec3;
use materials::{Lambertian, Metal};
use objects::{Sphere, World};
use std::rc::Rc;

mod camera;
mod colour;
mod hit;
mod interval;
mod materials;
mod objects;
mod ray;

const WIDTH: u32 = 1920;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const VERTICAL_FOV: f32 = 60.0;

const LOOK_FROM: Vec3 = Vec3::new(-0.7, 1.0, 0.6);
const LOOK_AT: Vec3 = Vec3::new(0.0, 0.5, -1.0);

const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u16 = 100;

const DEFOCUS_ANGLE: f32 = 2.0;
const FOCUS_DISTANCE: f32 = 1.2;

fn main() {
    let material_ground = Rc::new(Lambertian::new(rgb(0, 211, 255))) as _;
    let material_center = Rc::new(Lambertian::new(rgb(255, 211, 0))) as _;
    let material_left = Rc::new(Metal::new(rgb(127, 50, 0), 0.05)) as _;
    let material_right = Rc::new(Metal::new(rgb(50, 127, 50), 0.01)) as _;

    let world = World::new(vec![
        Rc::new(Sphere::new(
            Vec3::new(0.0, -100.0, -1.0),
            100.0,
            material_ground,
        )),
        Rc::new(Sphere::new(
            Vec3::new(-0.1, 0.5, -1.0),
            0.5,
            material_center,
        )),
        Rc::new(Sphere::new(Vec3::new(-1.2, 0.4, -1.0), 0.4, material_left)),
        Rc::new(Sphere::new(Vec3::new(1.1, 0.5, -0.3), 0.5, material_right)),
    ]);

    let mut camera = Camera::new();

    camera.render(&world);
}
