use super::vec3::{ Vec3 };
use super::hitable::{ Hitable };
use super::ray::Ray;
use std::f32::MAX;
use std::sync::Arc;
use super::camera::{ Camera };
use rand::prelude::*;

fn color<T: Hitable>(r: &Ray, world: &T, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, MAX) {
        let emitted = rec.material.emitted(0.0, 0.0, rec.p);
        if depth >= 50 { return emitted; }
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            emitted + attenuation * color(&scattered, world, depth + 1)
        } else {
            emitted
        }
    } else {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct Scene<'a> {
    pub starty: i32,
    pub endy: i32,
    pub nx: i32,
    pub ny: i32,
    pub ns: i32,
    pub cam: &'a Camera,
    pub hitable: Arc<Hitable>
}

pub fn render<T: Hitable>(scene: Scene) -> Vec<u8> {
    let mut file: Vec<u8> = Vec::with_capacity(
        (scene.endy - scene.starty) as usize *
        scene.nx as usize *
        3
    );
    let Scene { ns, nx, ny, cam, hitable, starty, endy } = scene;
    let mut rng = thread_rng();

    for j in (starty..endy).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &hitable, 0);
            };

            col /= ns as f32;
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();
            file.push((255.99 * col.x) as u8);
            file.push((255.99 * col.y) as u8);
            file.push((255.99 * col.z) as u8);
        }
    }
    file
}
