use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,

    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere{
            center: center,
            radius: radius,
            material: mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return None }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root { return None }
        }

        let hit_point = r.at(root);
        let normal = (hit_point - self.center) / self.radius;
        return Some(HitRecord::new(r, hit_point, normal, root, self.material.clone()));
    }
}
