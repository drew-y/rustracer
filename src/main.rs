mod bounding_box;
mod camera;
mod geometry;
mod hitable;
mod material;
mod perlin;
mod ray;
mod render;
#[allow(dead_code)]
mod scene;
mod texture;
mod utils;
mod vec3;

use image::png::PNGEncoder;
use render::render;
use scene::{earth, Scene};
use std::io;
use std::io::BufWriter;
use std::thread;

fn main() {
    let scene = earth();
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
        let render_thread = thread::spawn(move || render(thread_scene, starty, endy));
        render_threads.push(render_thread);
        endy = starty;
        starty -= y_section_size;
    }

    for render_thread in render_threads {
        file.extend(render_thread.join().unwrap());
    }

    let w = BufWriter::new(io::stdout());
    let encoder = PNGEncoder::new(w);
    match encoder.encode(&file, nx as u32, ny as u32, image::ColorType::RGB(8)) {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}
