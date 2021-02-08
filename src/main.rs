// mod printer;
mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod material;

use rand::random;
use vec3::{Vec3, Color};
use ray::Ray;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use material::{Material, Metal, Lambertian, Dielectric};
use camera::Camera;
use std::rc::Rc;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1920;
    let image_height: u32 = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;


    // camera
    let lookfrom = Vec3(13.0, 3.0, 10.0);
    let lookat = Vec3(0.5, 0.3, 0.0);
    let cam = Camera::new(
        lookfrom, lookat, Vec3(0.0, 1.0, 0.0),
        120.0, aspect_ratio,
        0.1, (lookat - lookfrom).length(),
    );

    // world
    let world = random_scene();

    // render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color: Color = Default::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            println!("{}", pixel_color.color_str(samples_per_pixel));
        }
    }
}



fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian(Vec3(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3(0.0, -1000.0, -1.0), 1000.0, ground_material,
    )));

    let diffuse = || {
        let albedo = Vec3::random(0.0, 1.0);
        Rc::new(Lambertian(albedo))
    };

    let metal = || {
        let albedo = Vec3::random(0.5, 1.0);
        let fuzz = random();
        Rc::new(Metal(albedo, fuzz))
    };

    let glass = || Rc::new(Dielectric(1.5));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let rnd = |x| x as f64 + 0.9*random::<f64>();
            let center = Vec3(rnd(a), 0.2, rnd(b));

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<dyn Material> = if choose_mat < 0.8 {
                    metal()
                } else if choose_mat < 0.95 {
                    diffuse()
                } else {
                    glass()
                };

                world.add(Box::new(Sphere::new(
                    center, 0.2, material,
                )));
            }
        }
    }

    let material1 = Rc::new(Dielectric(1.5));
    world.add(Box::new(Sphere::new(
        Vec3(0.0, 1.0, 0.0), 1.0, material1.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3(0.0, 1.0, 0.0), -0.9, material1.clone(),
    )));

    let material2 = Rc::new(Lambertian(Vec3(0.8, 0.4, 1.0)));
    world.add(Box::new(Sphere::new(
        Vec3(-3.0, 1.0, 0.0), 1.0, material2,
    )));

    let material3 = Rc::new(Metal(Vec3(0.7, 0.7, 0.7), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3(3.0, 1.0, 0.0), 1.0, material3,
    )));

    world
}


pub fn ray_color(r: &Ray, world: &impl Hittable, depth: u64) -> Color {
    if depth <= 0 {
        return Default::default();
    }

    if let Some(rec) = world.hit(r, (0.001, f64::INFINITY)) {

        if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }

        return Default::default()
    }

    let Vec3(_, y, _) = r.direction.unit_vec3();
    let t = 0.5 * (y + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}