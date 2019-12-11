extern crate rustracer;

use rand::prelude::*;
use rustracer::geometry::*;
use rustracer::material::{self, Material};
use rustracer::texture::*;
use rustracer::tracer::*;
use std::sync::Arc;

fn obsidian_world() -> Arc<dyn Hitable> {
    let mut list: Vec<BoxHitable> = Vec::new();
    let mut rng = thread_rng();
    let mut rand = || rng.gen::<f32>();

    // Make Randomly spaced black cubes
    let nb = 20;
    for i in 0..nb {
        for j in 0..nb {
            let width = 100;
            let x0 = -1000 + i * width;
            let z0 = -1000 + j * width;
            let y0 = 0;
            let x1 = x0 + width;
            let y1 = 100.0 * rand() + 0.01;
            let z1 = z0 + width;
            let cube = Cuboid::new(
                Vec3::new(x0 as f32, y0 as f32, z0 as f32),
                Vec3::new(x1 as f32, y1, z1 as f32),
                material::dielectric(1.5),
            );

            ConstantMedium {
                boundry: cube.box_clone(),
                density: 0.2,
                phase_function: material::isotropic(0.0, 0.0, 0.0),
            }
            .push_into_list_of_boxed_hitables(&mut list);
        }
    }

    Arc::new(BVHNode::new(list))
}

fn main() {
    // render(example_scene(), "./ray-tracing-the-next-week.png".into());
}
