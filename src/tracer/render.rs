use super::hitable::Hitable;
use super::ray::Ray;
use super::scene::{AnimatedScene, Scene};
use super::vec3::Vec3;
use rand::prelude::*;
use std::f32::MAX;

use image;
use indicatif::{ProgressBar, ProgressStyle};
use num_cpus;
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

fn sample_color(scene: &Scene, i: i32, j: i32) -> Vec3 {
    let Scene {
        ns,
        nx,
        cam,
        world,
        ny,
    } = scene;
    let mut rng = thread_rng();
    let nsf = *ns as f32;
    let nxf = *nx as f32;
    let nyf = *ny as f32;
    let if32 = i as f32;
    let jf32 = j as f32;
    let samples = (nsf as f32).sqrt() as i32;

    let mut col = Vec3::new(0.0, 0.0, 0.0);
    for s in 0..samples {
        for t in 0..samples {
            let u = ((s as f32 + rng.gen::<f32>()) / nsf + if32) / nxf;
            let v = ((t as f32 + rng.gen::<f32>()) / nsf + jf32) / nyf;
            let r = cam.get_ray(u, v);
            col += color(&r, world, 0);
        }
    }

    col /= nsf;
    col.sqrt()
}

fn render_section(scene: Scene, starty: i32, endy: i32, pb: ProgressBar) -> Vec<u8> {
    let mut file: Vec<u8> = Vec::with_capacity((endy - starty) as usize * scene.nx as usize * 3);
    let Scene { nx, .. } = scene;

    for j in (starty..endy).rev() {
        for i in 0..nx {
            let col = sample_color(&scene, i, j);
            file.push((255.99 * col.x).max(0.0).min(255.0) as u8);
            file.push((255.99 * col.y).max(0.0).min(255.0) as u8);
            file.push((255.99 * col.z).max(0.0).min(255.0) as u8);
        }
        pb.inc(1);
    }
    file
}

fn render_progress_bar(ny: i32) -> ProgressBar {
    let pb = ProgressBar::new(ny as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {msg} [{bar:50.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );
    pb.set_message("Rendering");
    pb.set_position(0);
    pb
}

pub fn render(scene: Scene, path: String) {
    let Scene { nx, ny, .. } = scene;
    let mut file: Vec<u8> = Vec::with_capacity((nx as usize) * (ny as usize) * 3);

    let thread_count = num_cpus::get();
    let mut render_threads: Vec<thread::JoinHandle<Vec<u8>>> = Vec::with_capacity(thread_count);
    let y_section_size = ny / thread_count as i32;
    let mut start_y = ny - y_section_size;
    let mut end_y = ny;
    let pb = render_progress_bar(ny);

    for _thread in 0..thread_count {
        let thread_scene = scene.clone();
        let thread_pb = pb.clone();
        let render_thread =
            thread::spawn(move || render_section(thread_scene, start_y, end_y, thread_pb));
        render_threads.push(render_thread);
        end_y = start_y;
        start_y -= y_section_size;
    }

    for render_thread in render_threads {
        file.extend(render_thread.join().unwrap());
    }

    // Render remaining y columns
    let remaining_y_columns = ny - (y_section_size * thread_count as i32);
    if remaining_y_columns > 0 {
        render_threads = Vec::with_capacity(remaining_y_columns as usize);
        for column in (0..remaining_y_columns).rev() {
            let thread_scene = scene.clone();
            let thread_pb = pb.clone();
            let render_thread =
                thread::spawn(move || render_section(thread_scene, column - 1, column, thread_pb));
            render_threads.push(render_thread);
        }

        for render_thread in render_threads {
            file.extend(render_thread.join().unwrap());
        }
    }

    pb.finish_with_message("Complete");

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
