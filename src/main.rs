mod aabb;
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

use camera::{Camera, CameraOpts};
use geometry::bvh::BVHNode;
use hitable::Hitable;
use image::png::PNGEncoder;
use render::{render, Scene};
use scene::rttnw_final_scene;
use std::io;
use std::io::BufWriter;
use std::sync::Arc;
use std::thread;
use vec3::Vec3;

fn main() {
    let nx: i32 = 400;
    let ny: i32 = 400;
    let ns: i32 = 400;
    let mut file: Vec<u8> = Vec::with_capacity((nx as usize) * (ny as usize) * 3);

    let world = Arc::new(BVHNode::new(rttnw_final_scene()));

    let cam = Camera::new(CameraOpts {
        lookfrom: Vec3::new(478.0, 278.0, -600.0),
        lookat: Vec3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: nx as f32 / ny as f32,
        focus_dist: 10.0,
        aperture: 0.0,
        vfow: 40.0,
    });

    let thread_count = 8;
    let mut render_threads: Vec<thread::JoinHandle<Vec<u8>>> =
        Vec::with_capacity(thread_count as usize);
    let y_section_size = ny / thread_count;
    let mut starty = ny - y_section_size;
    let mut endy = ny;
    for _render_thread_num in 0..thread_count {
        let thread_world = world.clone();
        let render_thread = thread::spawn(move || {
            render::<Arc<Hitable>>(Scene {
                nx,
                ny,
                ns,
                starty,
                endy,
                cam: &cam,
                hitable: thread_world,
            })
        });
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
