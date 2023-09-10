use super::{random_unit_sphere_vector, Material};
use crate::colour::Colour;
use crate::hit::HitRecord;
use crate::ray::Ray;

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut direction = rec.normal() + random_unit_sphere_vector();

        if direction.length_squared() < 0.0001 {
            direction = rec.normal();
        }

        let scattered = Ray::new(rec.point(), direction);

        Some((self.albedo, scattered))
    }
}
