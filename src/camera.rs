use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let Vec3(x, y, _) = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * x + self.v * y;
        let origin = self.origin + offset;
        Ray::new(
            origin,
            self.lower_left_corner + s*self.horizontal + t*self.vertical - origin,
        )
    }

    pub fn new(
        lookfrom: Point3, lookat: Point3, vup: Vec3,
        vfov: f64, aspect_ratio: f64,
        aperture: f64, focus_dist: f64,
    ) -> Camera {
        let h = (vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).unit_vec3();
        let u = vup.cross(&w).unit_vec3();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist*w,
            u: u, v: v,
            lens_radius: aperture / 2.0,
        }
    }
}
