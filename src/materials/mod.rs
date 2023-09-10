use crate::colour::Colour;
use crate::hit::HitRecord;
use crate::ray::Ray;
use glam::Vec3;
pub use lambertian::Lambertian;
pub use metal::Metal;

mod lambertian;
mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)>;
}

fn random_unit_sphere_vector() -> Vec3 {
    loop {
        let p = Vec3::new(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        ) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);

        if p.length_squared() < 1.0 {
            return p.normalize();
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}
