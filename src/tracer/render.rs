use super::hitable::Hitable;
use super::ray::Ray;
use super::scene::{AnimatedScene, Scene};
use super::vec3::Vec3;
use rand::prelude::*;
use std::f32::MAX;

use image;
use std::thread;

fn color(r: &Ray, world: &impl Hitable, depth: i32) -> Vec3 {
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

fn render_section(scene: Scene, starty: i32, endy: i32) -> Vec<u8> {
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

pub fn render(scene: Scene, path: String) {
    let Scene { nx, ny, .. } = scene;
    let mut file: Vec<u8> = Vec::with_capacity((nx as usize) * (ny as usize) * 3);

    let thread_count = 8;
    let mut render_threads: Vec<thread::JoinHandle<Vec<u8>>> =
        Vec::with_capacity(thread_count as usize);
    let y_section_size = ny / thread_count;
    let mut starty = ny - y_section_size;
    let mut endy = ny;

    for _render_thread_num in 0..thread_count {
        let thread_scene = scene.clone();
        let render_thread = thread::spawn(move || render_section(thread_scene, starty, endy));
        render_threads.push(render_thread);
        endy = starty;
        starty -= y_section_size;
    }

    for render_thread in render_threads {
        file.extend(render_thread.join().unwrap());
    }

    match image::save_buffer(path, &file, nx as u32, ny as u32, image::ColorType::RGB(8)) {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}

pub fn render_animation(scene: AnimatedScene, path: String) {
    let time_step = 1.0 / scene.fps;
    let mut time = scene.start;

    while time <= scene.end {
        render(
            (scene.scene_fn)(time),
            format!("./{}/frame-{}.png", path, time),
        );
        time += time_step;
    }
}
