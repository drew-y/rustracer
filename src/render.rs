use super::hitable::Hitable;
use super::ray::Ray;
use super::scene::Scene;
use super::vec3::Vec3;
use rand::prelude::*;
use std::f32::MAX;

#[allow(dead_code)]
fn sky_background(r: &Ray) -> Vec3 {
    let unit_direction = &r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn color<T: Hitable>(r: &Ray, world: &T, depth: i32) -> Vec3 {
    let rec = match world.hit(r, 0.001, MAX) {
        Some(rec) => rec,
        None => return Vec3::new(0.0, 0.0, 0.0),
    };

    let emitted = rec.material.emitted(rec.u, rec.v, rec.p);
    if depth >= 50 {
        return emitted;
    }

    let (attenuation, scattered) = match rec.material.scatter(r, &rec) {
        Some((attenuation, scattered)) => (attenuation, scattered),
        None => return emitted,
    };

    emitted + attenuation * color(&scattered, world, depth + 1)
}

pub fn render(scene: Scene, starty: i32, endy: i32) -> Vec<u8> {
    let mut file: Vec<u8> = Vec::with_capacity((endy - starty) as usize * scene.nx as usize * 3);
    let Scene {
        ns,
        nx,
        ny,
        cam,
        world,
    } = scene;
    let mut rng = thread_rng();

    for j in (starty..endy).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }

            col /= ns as f32;
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();
            file.push((255.99 * col.x).max(0.0).min(255.0) as u8);
            file.push((255.99 * col.y).max(0.0).min(255.0) as u8);
            file.push((255.99 * col.z).max(0.0).min(255.0) as u8);
        }
    }
    file
}
