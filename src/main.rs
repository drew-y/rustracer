mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;
mod utils;
mod bvh;
mod aabb;
mod texture;
mod scene;
mod render;

use vec3::{ Vec3 };
use hitable::{ Hitable };
use std::thread;
use std::sync::Arc;
use camera::{ Camera, CameraOpts };
use bvh::BVHNode;
use std::io;
use std::io::BufWriter;
use png::HasParameters;
use scene::random_scene;
use render::{ Scene, render };

fn main() {
    let nx: i32 = 1200;
    let ny: i32 = 800;
    let ns: i32 = 10;
    let mut file: Vec<u8> = Vec::with_capacity((nx as usize) * (ny as usize) * 3);

    let world = Arc::new(BVHNode::new(random_scene()));

    let cam = Camera::new(CameraOpts {
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: nx as f32 / ny as f32,
        focus_dist: 10.0, aperture: 0.1, vfow: 20.0
    });

    let thread_count = 8;
    let mut render_threads: Vec<thread::JoinHandle<Vec<u8>>> =
        Vec::with_capacity(thread_count as usize);
    let y_section_size = ny / thread_count;
    let mut starty = ny - y_section_size;
    let mut endy = ny;
    for _render_thread_num in 0..thread_count {
        let thread_world = world.clone();
        let render_thread = thread::spawn(move || render::<Arc<Hitable>>(Scene {
            nx, ny, ns, starty, endy,
            cam: &cam,
            hitable: thread_world
        }));
        render_threads.push(render_thread);
        endy = starty;
        starty -= y_section_size;
    }

    for render_thread in render_threads {
        file.extend(render_thread.join().unwrap());
    }

    let ref mut w = BufWriter::new(io::stdout());
    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&file).unwrap();
}
