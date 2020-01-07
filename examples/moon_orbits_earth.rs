#[macro_use]
extern crate lazy_static;
extern crate rustracer;

use rustracer::animation::*;
use rustracer::geometry::*;
use rustracer::material;
use rustracer::tracer::*;
use std::sync::Arc;

lazy_static! {
    static ref EARTHMAP: material::Material = material::lambertion_with_image("../earthmap.jpg");
    static ref MOONMAP: material::Material = material::lambertion_with_image("../moonmap.jpg");
}

fn earth(time: f32) -> Image {
    let mut list: Vec<Box<dyn Hitable>> = Vec::with_capacity(2);

    // The Sun
    Sphere {
        center: Vec3::new(20.0, 9.0, 8.0),
        radius: 6.0,
        material: material::diffuse_light(10.0, 10.0, 10.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // The Earth
    Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 3.5,
        material: EARTHMAP.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Earth's atmosphere
    ConstantMedium {
        boundry: Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 3.51,
            material: material::lambertion(0.0, 0.0, 0.0),
        }
        .box_clone(),
        density: 20.0,
        phase_function: material::isotropic(0.45, 0.77, 0.9999),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let moon_orbit = Orbit3D::new(Vec3::new(0.0, 1.0, 5.0), Vec3::new(0.0, 0.0, 0.0), 72.0);

    // The Moon
    Sphere {
        center: moon_orbit.point_at_time(time),
        radius: 0.6,
        material: MOONMAP.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let world = Arc::new(BVHNode::new(list));

    let nx: i32 = 600;
    let ny: i32 = 600;
    let ns: i32 = 1000;
    let cam = Camera::new(CameraOpts {
        lookfrom: Vec3::new(0.0, 0.0, 15.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: nx as f32 / ny as f32,
        focus_dist: 13.0,
        aperture: 0.0,
        vfow: 40.0,
    });

    Image {
        width: nx,
        height: ny,
        samples: ns,
        cam,
        world,
    }
}

pub fn moon_orbits_earth() {
    let renderer = AnimationRenderer::from(Animation {
        fps: 60.0,
        start_time: 0.0,
        end_time: 6.0,
        image_fn: &earth,
    });
    renderer.render("moon_orbits_earth");
}

fn main() {
    moon_orbits_earth();
}
