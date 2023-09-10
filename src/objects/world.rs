use crate::hit::{HitRecord, Hittable};
use crate::interval::{Interval, Range};
use crate::ray::Ray;
use std::rc::Rc;

pub struct World {
    objects: Vec<Rc<dyn Hittable>>,
}

impl World {
    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> World {
        World { objects }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, ray_t: Range) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max();

        let mut hit_record = None;

        for object in &self.objects {
            if let Some(record) = object.hit(ray, ray_t.min()..closest_so_far) {
                closest_so_far = record.t();
                hit_record = Some(record);
            }
        }

        hit_record
    }
}
