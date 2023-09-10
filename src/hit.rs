use std::rc::Rc;

use crate::interval::Range;
use crate::materials::Material;
use crate::ray::Ray;
use glam::Vec3;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f32,
    // front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f32,
        ray: &Ray,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        let front_face = ray.direction().dot(normal) < 0.0;

        HitRecord {
            point,
            normal: if front_face { normal } else { -normal },
            material,
            t,
        }
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }

    pub fn t(&self) -> f32 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Range) -> Option<HitRecord>;
}
