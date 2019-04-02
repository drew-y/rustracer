mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;

use vec3::{ Vec3, unit_vector };
use hitable::{ Hitable, HitableList };
use ray::Ray;
use std::f64::MAX;
use sphere::{ Sphere };
use camera::Camera;
use rand::Rng;
use material::{ Material };

fn color<T: Hitable>(r: &Ray, world: &T, depth: i64) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, MAX) {
        if depth >= 50 { return Vec3::new(0.0, 0.0, 0.0); }
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * color(&scattered, world, depth + 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 1600;
    let ny = 800;
    let ns = 100;
    let mut rng = rand::thread_rng();
    println!("P3\n{} {}\n255\n", nx, ny);

    let world = HitableList { list: vec![
        Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Lambertion { albedo: Vec3::new(0.8, 0.3, 0.3) }
        },
        Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Material::Lambertion { albedo: Vec3::new(0.8, 0.8, 0.0) }
        },
        Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 0.3 }
        },
        Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Dielectric { ref_idx: 1.5 }
        },
        Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Material::Dielectric { ref_idx: 1.5 }
        },
    ] };

    let cam = Camera::default();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(nx);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(ny);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            };

            col /= f64::from(ns);
            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
