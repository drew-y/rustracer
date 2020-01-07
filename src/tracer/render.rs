use super::helpers::{make_progress_bar, RenderThread};
use super::hitable::{Hitable, World};
use super::ray::Ray;
use super::vec3::Vec3;
use crate::tracer::camera::Camera;
use indicatif::ProgressBar;
use rand::prelude::*;
use std::f32::MAX;

use image;
use num_cpus;
use std::thread;

#[derive(Clone)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub samples: i32,
    pub cam: Camera,
    pub world: World,
}

#[derive(Clone)]
pub struct Renderer {
    nx: i32,
    ny: i32,
    ns: i32,
    cam: Camera,
    world: World,
    pb: ProgressBar,
}

impl From<Image> for Renderer {
    fn from(image: Image) -> Self {
        Renderer {
            pb: make_progress_bar("Rendering", image.width),
            ns: image.samples,
            ny: image.height,
            nx: image.width,
            cam: image.cam,
            world: image.world,
        }
    }
}

impl Renderer {
    fn color(&self, r: &Ray, depth: i32) -> Vec3 {
        let rec = match self.world.hit(r, 0.001, MAX) {
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

        emitted + attenuation * self.color(&scattered, depth + 1)
    }

    fn sample_color(&self, i: i32, j: i32) -> Vec3 {
        let mut rng = thread_rng();
        let nsf = self.ns as f32;
        let nxf = self.nx as f32;
        let nyf = self.ny as f32;

        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _s in 0..self.ns {
            let u = (i as f32 + rng.gen::<f32>()) / nxf;
            let v = (j as f32 + rng.gen::<f32>()) / nyf;
            let r = self.cam.get_ray(u, v);
            col += self.color(&r, 0);
        }

        col /= nsf;
        col.sqrt()
    }

    fn render_section(&self, start_y: i32, end_y: i32) -> Vec<u8> {
        let mut file: Vec<u8> =
            Vec::with_capacity((start_y - end_y) as usize * self.nx as usize * 3);

        for j in (end_y..start_y).rev() {
            for i in 0..self.nx {
                let col = self.sample_color(i, j);
                file.push((255.99 * col.x).max(0.0).min(255.0) as u8);
                file.push((255.99 * col.y).max(0.0).min(255.0) as u8);
                file.push((255.99 * col.z).max(0.0).min(255.0) as u8);
            }
            self.pb.inc(1);
        }
        file
    }

    fn spawn_render_thread(&self, start_y: i32, end_y: i32) -> RenderThread {
        let this = self.clone();
        thread::spawn(move || this.render_section(start_y, end_y))
    }

    pub fn render(&self, path: impl std::convert::AsRef<std::path::Path>) {
        let mut file: Vec<u8> = Vec::with_capacity((self.nx as usize) * (self.ny as usize) * 3);
        let thread_count = num_cpus::get();
        let y_section_size = self.ny / thread_count as i32;

        let mut render_threads: Vec<RenderThread> = Vec::with_capacity(thread_count);
        for (start_y, end_y) in Self::generate_y_sections(self.ny, y_section_size) {
            render_threads.push(self.spawn_render_thread(start_y, end_y));
        }
        Self::join_render_threads_into_file(render_threads, &mut file);

        let remaining_y_columns = self.ny % y_section_size;
        if remaining_y_columns > 0 {
            render_threads = Vec::with_capacity(remaining_y_columns as usize);
            for column in (0..remaining_y_columns).rev() {
                render_threads.push(self.spawn_render_thread(column + 1, column));
            }

            Self::join_render_threads_into_file(render_threads, &mut file);
        }

        match image::save_buffer(
            path,
            &file,
            self.nx as u32,
            self.ny as u32,
            image::ColorType::RGB(8),
        ) {
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            _ => {}
        }
    }
}

/// Helpers
impl Renderer {
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

    pub fn override_progress_bar(&mut self, pb: ProgressBar) {
        self.pb = pb;
    }
}
