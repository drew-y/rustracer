extern crate rustracer;

use rand::prelude::*;
use rustracer::geometry::*;
use rustracer::material::{self, Material};
use rustracer::texture::*;
use rustracer::tracer::*;
use std::sync::Arc;

fn rttnw_final_world() -> Arc<dyn Hitable> {
    let mut list: Vec<BoxHitable> = Vec::with_capacity(429);
    let mut rng = thread_rng();
    let mut rand = || rng.gen::<f32>();
    let white = material::lambertion(0.73, 0.73, 0.73);
    let ground = material::lambertion(0.48, 0.83, 0.53);
    let light = material::diffuse_light(7.0, 7.0, 7.0);

    let nb = 20;
    for i in 0..nb {
        for j in 0..nb {
            let w = 100;
            let x0 = -1000 + i * w;
            let z0 = -1000 + j * w;
            let y0 = 0;
            let x1 = x0 + w;
            let y1 = 100.0 * rand() + 0.01;
            let z1 = z0 + w;
            Cuboid::new(
                Vec3::new(x0 as f32, y0 as f32, z0 as f32),
                Vec3::new(x1 as f32, y1, z1 as f32),
                ground.clone(),
            )
            .push_into_list_of_boxed_hitables(&mut list);
        }
    }

    XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: light.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: material::dielectric(1.5),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: material::metal(Vec3::new(0.8, 0.8, 0.9), 0.7),
    }
    .push_into_list_of_boxed_hitables(&mut list);

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

    let boundry2 = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5000.0,
        material: material::dielectric(1.5),
    };

    ConstantMedium {
        boundry: boundry2.box_clone(),
        density: 0.0001,
        phase_function: material::isotropic(1.0, 1.0, 1.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: material::lambertion_with_image("./earthmap.jpg"),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Material::Lambertion {
            albedo: NoiseTexture::new(0.1).box_clone(),
        },
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let mut list2: Vec<BoxHitable> = Vec::with_capacity(429);
    for _ in 0..1000 {
        Sphere {
            center: Vec3::new(165.0 * rand(), 165.0 * rand(), 165.0 * rand()),
            radius: 10.0,
            material: white.clone(),
        }
        .push_into_list_of_boxed_hitables(&mut list2);
    }

    BVHNode::new(list2)
        .rotate_y(15.0)
        .shift(-100.0, 270.0, 395.0)
        .push_into_list_of_boxed_hitables(&mut list);

    Arc::new(BVHNode::new(list))
}

/// An example scene from
pub fn example_scene() -> Scene {
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
        world: rttnw_final_world(),
    }
}

fn main() {
    render(example_scene(), "./ray-tracing-the-next-week.png".into());
}
