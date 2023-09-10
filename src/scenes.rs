use crate::colour::rgb;
use crate::materials::{Lambertian, Metal};
use crate::objects::{Sphere, World};
use glam::Vec3;
use std::rc::Rc;

fn ground_material() -> Lambertian {
    Lambertian::new(rgb(0, 211, 255))
}

fn ground_sphere() -> Sphere {
    Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(ground_material()),
    )
}

pub fn three_spheres() -> World {
    let material_center = Rc::new(Lambertian::new(rgb(255, 211, 0)));
    let material_left = Rc::new(Metal::new(rgb(127, 50, 0), 0.05));
    let material_right = Rc::new(Metal::new(rgb(50, 127, 50), 0.01));

    World::new(vec![
        Rc::new(ground_sphere()),
        Rc::new(Sphere::new(
            Vec3::new(-0.5, 0.0, -1.0),
            0.5,
            material_center,
        )),
        Rc::new(Sphere::new(Vec3::new(-1.6, -0.1, -0.8), 0.4, material_left)),
        Rc::new(Sphere::new(Vec3::new(0.7, 0.0, -0.5), 0.5, material_right)),
    ])
}

pub fn rainbow_spheres() -> World {
    const RADIUS: f32 = 0.15;

    let material_red = Rc::new(Metal::new(rgb(255, 0, 0), 0.1));
    let material_orange = Rc::new(Metal::new(rgb(255, 127, 0), 0.1));
    let material_yellow = Rc::new(Metal::new(rgb(255, 255, 0), 0.1));
    let material_green = Rc::new(Metal::new(rgb(0, 255, 0), 0.1));
    let material_blue = Rc::new(Metal::new(rgb(0, 0, 255), 0.1));
    let material_indigo = Rc::new(Metal::new(rgb(75, 0, 130), 0.1));

    World::new(vec![
        Rc::new(ground_sphere()),
        Rc::new(Sphere::new(
            Vec3::new(-0.5, 1.0 - 0.5 + RADIUS, 0.0),
            RADIUS,
            material_red,
        )),
        Rc::new(Sphere::new(
            Vec3::new(-0.25, 0.8 - 0.5 + RADIUS, 0.2),
            RADIUS,
            material_orange,
        )),
        Rc::new(Sphere::new(
            Vec3::new(0.0, 0.6 - 0.5 + RADIUS, 0.4),
            RADIUS,
            material_yellow,
        )),
        Rc::new(Sphere::new(
            Vec3::new(0.25, 0.4 - 0.5 + RADIUS, 0.6),
            RADIUS,
            material_green,
        )),
        Rc::new(Sphere::new(
            Vec3::new(0.5, 0.2 - 0.5 + RADIUS, 0.8),
            RADIUS,
            material_blue,
        )),
        Rc::new(Sphere::new(
            Vec3::new(0.75, 0.0 - 0.5 + RADIUS, 1.0),
            RADIUS,
            material_indigo,
        )),
    ])
}
