use super::{random_unit_sphere_vector, reflect, Material};
use crate::colour::Colour;
use crate::hit::HitRecord;
use crate::ray::Ray;

pub struct Metal {
    albedo: Colour,
    roughness: f32,
}

impl Metal {
    pub fn new(albedo: Colour, roughness: f32) -> Self {
        Self { albedo, roughness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = reflect(ray.direction().normalize(), rec.normal());

        let scattered = Ray::new(
            rec.point(),
            reflected + self.roughness * random_unit_sphere_vector(),
        );

        if scattered.direction().dot(rec.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
