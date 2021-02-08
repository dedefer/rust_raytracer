use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn new() -> HittableList { HittableList(Vec::new()) }
    pub fn add(&mut self, el: Box<dyn Hittable>) { self.0.push(el) }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for el in self.0.iter() {
            if let Some(rec) = el.hit(r, (t_min, closest_so_far)) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
