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

type RenderThread = thread::JoinHandle<Vec<u8>>;

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

    let mut col = Vec3::new(0.0, 0.0, 0.0);
    for _s in 0..*ns {
        let u = (i as f32 + rng.gen::<f32>()) / nxf;
        let v = (j as f32 + rng.gen::<f32>()) / nyf;
        let r = cam.get_ray(u, v);
        col += color(&r, world, 0);
    }

    col /= nsf;
    col.sqrt()
}

fn render_section(scene: Scene, start_y: i32, end_y: i32, pb: ProgressBar) -> Vec<u8> {
    let mut file: Vec<u8> = Vec::with_capacity((start_y - end_y) as usize * scene.nx as usize * 3);
    let Scene { nx, .. } = scene;

    for j in (end_y..start_y).rev() {
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

fn make_render_progress_bar(ny: i32) -> ProgressBar {
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

fn spawn_render_thread(scene: &Scene, pb: &ProgressBar, start_y: i32, end_y: i32) -> RenderThread {
    let thread_scene = scene.clone();
    let thread_pb = pb.clone();
    thread::spawn(move || render_section(thread_scene, start_y, end_y, thread_pb))
}

fn join_render_threads_into_file(threads: Vec<RenderThread>, file: &mut Vec<u8>) {
    for render_thread in threads {
        file.extend(render_thread.join().unwrap());
    }
}

/// Generate y sections of in image as (start_y, end_y). Always counts downwards.
fn generate_y_sections(ny: i32, section_size: i32) -> Vec<(i32, i32)> {
    let mut start_y = ny;
    let mut end_y = start_y - section_size;
    let mut vec: Vec<(i32, i32)> = Vec::new();

    while end_y >= 0 {
        vec.push((start_y, end_y));
        start_y = end_y;
        end_y -= section_size;
    }

    vec
}

pub fn render(scene: Scene, path: String) {
    let Scene { nx, ny, .. } = scene;
    let mut file: Vec<u8> = Vec::with_capacity((nx as usize) * (ny as usize) * 3);
    let pb = make_render_progress_bar(ny);
    let thread_count = num_cpus::get();
    let y_section_size = ny / thread_count as i32;

    let mut render_threads: Vec<RenderThread> = Vec::with_capacity(thread_count);
    for (start_y, end_y) in generate_y_sections(ny, y_section_size) {
        render_threads.push(spawn_render_thread(&scene, &pb, start_y, end_y));
    }
    join_render_threads_into_file(render_threads, &mut file);

    let remaining_y_columns = ny % y_section_size;
    if remaining_y_columns > 0 {
        render_threads = Vec::with_capacity(remaining_y_columns as usize);
        for column in (0..remaining_y_columns).rev() {
            render_threads.push(spawn_render_thread(&scene, &pb, column + 1, column));
        }

        join_render_threads_into_file(render_threads, &mut file);
    }

    match image::save_buffer(path, &file, nx as u32, ny as u32, image::ColorType::RGB(8)) {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }

    pb.finish_with_message("Complete");
}

pub fn render_animation(scene: AnimatedScene, path: String) {
    let time_step = 1.0 / scene.fps;
    let mut time = scene.start;

    let mut frame = (time / time_step) as i32 + 1;
    while time <= scene.end {
        render(
            (scene.scene_fn)(time),
            format!("./{}/frame-{}.png", path, frame),
        );
        time += time_step;
        frame += 1;
    }
}
