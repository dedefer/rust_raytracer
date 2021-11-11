use crate::vec3::{Color, Vec3};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use rand::random;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone, Copy)]
pub struct Lambertian(pub Color);

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        Some((Ray::new(rec.point, scatter_direction), self.0))
    }
}

#[derive(Clone, Copy)]
pub struct Metal(pub Color, pub f64);


impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&r_in.direction.unit_vec3(), &rec.normal);
        let scattered = Ray::new(rec.point, reflected + self.1 * Vec3::random_in_unit_sphere());
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((scattered, self.0))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric(pub f64);

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.0,
            false => self.0,
        };

        let unit_direction = r_in.direction.unit_vec3();

        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cant_refract = refraction_ratio * sin_theta > 1.0;
        let direction = match cant_refract || Self::reflectance(cos_theta, refraction_ratio) > random::<f64>() {
            true => reflect(&unit_direction, &rec.normal),
            false => refract(&unit_direction, &rec.normal, refraction_ratio),
        };

        Some((
            Ray::new(rec.point, direction),
            Vec3(1.0, 1.0, 1.0),
        ))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv.dot(n)).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
    r_out_perp + r_out_parallel
}

// class metal : public material {
//     public:
//         metal(const color& a) : albedo(a) {}

//         virtual bool scatter(
//             const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
//         ) const override {
//             vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
//             scattered = ray(rec.p, reflected);
//             attenuation = albedo;
//             return (dot(scattered.direction(), rec.normal) > 0);
//         }

//     public:
//         color albedo;
// };
