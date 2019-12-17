extern crate rustracer;

use rand::prelude::*;
use rustracer::geometry::*;
use rustracer::material::{self};
// use rustracer::texture::*;
use rustracer::tracer::*;
use std::sync::Arc;

fn obsidian_world() -> Arc<dyn Hitable> {
    let mut list: Vec<BoxHitable> = Vec::new();
    let mut rng = thread_rng();
    let mut rand = || rng.gen::<f32>();
    let light = material::diffuse_light(7.0, 7.0, 7.0);

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
                phase_function: material::isotropic(0.1, 0.1, 0.1),
            }
            .push_into_list_of_boxed_hitables(&mut list);
            cube.push_into_list_of_boxed_hitables(&mut list);
        }
    }

    // The main light
    XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: light.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Blue Sphere
    let boundry1 = Sphere {
        center: Vec3::new(360.0, 150.0, 145.0),
        radius: 50.0,
        material: material::dielectric(1.5),
    };

    ConstantMedium {
        boundry: boundry1.box_clone(),
        density: 0.2,
        phase_function: material::isotropic(0.2, 0.4, 0.9),
    }
    .push_into_list_of_boxed_hitables(&mut list);
    boundry1.push_into_list_of_boxed_hitables(&mut list);

    // Black Sphere
    let boundry2 = Sphere {
        center: Vec3::new(150.0, 150.0, 145.0),
        radius: 50.0,
        material: material::dielectric(1.5),
    };

    ConstantMedium {
        boundry: boundry2.box_clone(),
        density: 0.2,
        phase_function: material::isotropic(0.0, 0.0, 0.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);
    boundry2.push_into_list_of_boxed_hitables(&mut list);

    Arc::new(BVHNode::new(list))
}

fn obsidian_scene() -> Scene {
    let nx: i32 = 400;
    let ny: i32 = 400;
    let ns: i32 = 40;
    let cam = Camera::new(CameraOpts {
        lookfrom: Vec3::new(478.0, 278.0, -600.0),
        lookat: Vec3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: nx as f32 / ny as f32,
        focus_dist: 10.0,
        aperture: 0.0,
        vfow: 40.0,
    });

    Scene {
        nx,
        ny,
        ns,
        cam,
        world: obsidian_world(),
    }
}

fn main() {
    render(obsidian_scene(), "./obsidian.png".into());
}
