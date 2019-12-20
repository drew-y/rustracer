extern crate rustracer;

use rustracer::animation::*;
use rustracer::geometry::*;
use rustracer::material::{self};
use rustracer::tracer::*;
use std::sync::Arc;

type List = Vec<BoxHitable>;

fn cube_light(center: Vec3, list: &mut List) {
    let brightness = center.y / 100.0;
    Cuboid::cube(
        12.0,
        center,
        material::diffuse_light(10.0 * brightness, 9.8 * brightness, 6.3 * brightness),
    )
    .push_into_list_of_boxed_hitables(list);
}

fn cube_grid(list: &mut List, time: f32) {
    for x in -150..150 {
        for z in -150..150 {
            if (x as i32).abs() <= 2 && (z as i32).abs() <= 2 {
                continue;
            }
            cube_light(
                Vec3::new(
                    x as f32 * 100.0,
                    60.0 * ((time / 2.0) + (x as f32).abs() + (z as f32).abs())
                        .sin()
                        .abs(),
                    z as f32 * 100.0,
                ),
                list,
            );
        }
    }
}

fn ghostly_orbs(time: f32) -> Arc<dyn Hitable> {
    let mut list: List = Vec::new();
    let light = material::diffuse_light(7.0, 7.0, 7.0);
    let red_light = material::diffuse_light(0.9, 0.2, 0.2);

    // Main Light
    XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: light.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Floor
    let floor = Cuboid::new(
        Vec3::new(-1000.0, -300.0, -1000.0),
        Vec3::new(1000.0, 0.0, 1000.0),
        material::dielectric(1.5),
    );

    ConstantMedium {
        boundry: floor.box_clone(),
        density: 0.2,
        phase_function: material::isotropic(0.0, 0.0, 0.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);
    floor.push_into_list_of_boxed_hitables(&mut list);

    cube_grid(&mut list, time);

    // Glass Sphere
    Sphere {
        center: Vec3::new(0.0, 55.0, 0.0),
        radius: 50.0,
        material: material::dielectric(1.5),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let red_orbit = Orbit3D::new(Vec3::new(23.0, 60.0, 0.0), Vec3::new(0.0, 55.0, 0.0), 72.0);

    // Glowing Red Ball inside Glass Sphere
    Sphere {
        center: red_orbit.point_at_time(time),
        radius: 5.0,
        material: red_light.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Mirror core of sphere
    Sphere {
        center: Vec3::new(0.0, 55.0, 0.0),
        radius: 9.0,
        material: material::metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Background
    Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2000.0,
        material: material::lambertion(0.1, 0.1, 0.2),
    }
    .flip_normals()
    .push_into_list_of_boxed_hitables(&mut list);

    Arc::new(BVHNode::new(list))
}

fn ghostly_orbs_scene(time: f32) -> Scene {
    let nx: i32 = 400;
    let ny: i32 = 400;
    let ns: i32 = 2;

    let camera_orbit = Orbit3D::new(
        Vec3::new(200.0, 200.0, -700.0),
        Vec3::new(0.0, 50.0, 0.0),
        1.0,
    );

    let cam = Camera::new(CameraOpts {
        lookfrom: camera_orbit.point_at_time(time),
        lookat: Vec3::new(0.0, 50.0, 0.0),
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
        world: ghostly_orbs(time),
    }
}

fn main() {
    render_animation(
        AnimatedScene {
            fps: 24,
            duration: 6,
            scene_fn: &ghostly_orbs_scene,
        },
        "ghostly_orbs".into(),
    );
}
