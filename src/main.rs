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
use rand::prelude::*;
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

fn random_scene() -> HitableList<Sphere> {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f64>();
    let fl = |i: i32| f64::from(i);
    let mut list = HitableList {
        list: vec![
            Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Lambertion { albedo: Vec3::new(0.5, 0.5, 0.5) }
            },
        ]
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rnd();
            let center = Vec3::new(fl(a) + 0.9 * rnd(), 0.2, fl(b) + 0.9 * rnd());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // Diffuse
                if choose_mat < 0.8 {
                    list.list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertion { albedo: Vec3::new(rnd() * rnd(), rnd() * rnd(), rnd() * rnd()) }
                    });
                    continue;
                };

                if choose_mat < 0.95 { // Metal
                    list.list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal {
                            albedo: Vec3::new(0.5 * (1.0 + rnd()), 0.5 * (1.0 + rnd()), 0.5 * (1.0 + rnd())),
                            fuzz: 0.5 * rnd()
                        }
                    });
                    continue;
                };

                // Glass
                list.list.push(Sphere {
                    center, radius: 0.2,
                    material: Material::Dielectric { ref_idx: 1.5 }
                })
            };
        };
    };

    list.list.push(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0,
        material: Material::Dielectric { ref_idx: 1.5 }
    });

    list.list.push(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0,
        material: Material::Lambertion { albedo: Vec3::new(0.4, 0.2, 0.1) }
    });

    list.list.push(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0,
        material: Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0
        }
    });

    list
}

fn main() {
    let nx = 1600;
    let ny = 800;
    let ns = 50;
    let mut rng = thread_rng();
    println!("P3\n{} {}\n255\n", nx, ny);

    let world = random_scene();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom, lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(nx) / f64::from(ny),
        aperture, dist_to_focus
    );

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
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();
            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
