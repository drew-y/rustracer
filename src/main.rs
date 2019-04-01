mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;

use vec3::{ Vec3, unit_vector };
use hitable::{ Hitable, HitableList };
use ray::Ray;
use std::f64::MAX;
use sphere::Sphere;
use camera::Camera;

fn color<T: Hitable>(r: &Ray, world: &T) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, MAX) {
        return 0.5 * Vec3::new(
            rec.normal.x + 1.0,
            rec.normal.y + 1.0,
            rec.normal.z + 1.0
        );
    }

    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = HitableList { list: vec![
        Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 },
        Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 }
    ] };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = f64::from(i) / f64::from(nx);
            let v = f64::from(j) / f64::from(ny);
            let r = Ray { origin,
                direction: lower_left_corner + u * horizontal + v * vertical
            };

            let col = color(&r, &world);
            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
