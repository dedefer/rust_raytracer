use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,

    pub front_face: bool,
}

impl HitRecord {
    pub fn new(r: &Ray, point: Point3, outward_normal: Vec3, t: f64, mat: Arc<dyn Material>) -> HitRecord {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        HitRecord{
            t: t,
            point: point,
            normal: if front_face { outward_normal } else { -outward_normal },
            front_face: front_face,

            material: mat,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t: (f64, f64)) -> Option<HitRecord>;
}
