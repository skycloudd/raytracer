use std::rc::Rc;

use crate::hit::{HitRecord, Hittable};
use crate::interval::Range;
use crate::materials::Material;
use crate::ray::Ray;
use glam::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            let mut root = (-half_b - sqrtd) / a;

            if !ray_t.contains(&root) {
                root = (-half_b + root) / a;

                if !ray_t.contains(&root) {
                    return None;
                }
            }

            let point = ray.at(root);
            let normal = (point - self.center) / self.radius;

            Some(HitRecord::new(
                point,
                normal,
                root,
                ray,
                Rc::clone(&self.material),
            ))
        }
    }
}
