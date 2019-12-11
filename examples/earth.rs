extern crate rustracer;

use rustracer::geometry::*;
use rustracer::material;
use rustracer::tracer::*;
use std::sync::Arc;

pub fn earth() -> Scene {
    let mut list: Vec<Box<dyn Hitable>> = Vec::with_capacity(2);

    // The Sun
    Sphere {
        center: Vec3::new(8.0, 9.0, 20.0),
        radius: 6.0,
        material: material::diffuse_light(10.0, 10.0, 10.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // The Earth
    Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 3.5,
        material: material::lambertion_with_image("./earthmap.jpg"),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Earth's atmosphere
    ConstantMedium {
        boundry: Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 3.51,
            material: material::lambertion(0.0, 0.0, 0.0),
        },
        density: 20.0,
        phase_function: material::isotropic(0.45, 0.77, 0.9999),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // The Moon
    Sphere {
        center: Vec3::new(6.0, 2.0, -1.2),
        radius: 0.6,
        material: material::lambertion_with_image("./moonmap.jpg"),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let world = Arc::new(BVHNode::new(list));

    let nx: i32 = 800;
    let ny: i32 = 800;
    let ns: i32 = 50;
    let cam = Camera::new(CameraOpts {
        lookfrom: Vec3::new(15.0, 3.0, 2.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: nx as f32 / ny as f32,
        focus_dist: 13.0,
        aperture: 0.0,
        vfow: 40.0,
    });

    Scene {
        nx,
        ny,
        ns,
        cam,
        world,
    }
}

fn main() {
    render(earth(), "./earth.png".into());
}
