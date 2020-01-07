extern crate rustracer;

use rustracer::animation::*;
use rustracer::geometry::*;
use rustracer::material::{self};
use rustracer::tracer::*;
use std::sync::Arc;

type List = Vec<BoxHitable>;
static GLASS_SPHERE_RADIUS: f32 = 75.0;

fn cube_light(center: Vec3, list: &mut List) {
    let brightness = center.y * 2.0 / 100.0 + 0.01;
    Cuboid::cube(
        12.0,
        center,
        material::diffuse_light(1.5 * brightness, 1.5 * brightness, 1.5 * brightness),
    )
    .push_into_list_of_boxed_hitables(list);
}

fn cube_grid(list: &mut List, time: f32) {
    for x in -48..48 {
        for z in -48..48 {
            let distance_increment = 75.0;
            let x = x as f32 * distance_increment;
            let z = z as f32 * distance_increment;
            let distance = (x.powi(2) + z.powi(2)).sqrt();

            if distance + 3.0 < GLASS_SPHERE_RADIUS {
                continue;
            }

            let y = (distance * 0.005 - (time + 2.0) * 2.5).cos() * 100.0 - 30.0;
            cube_light(Vec3::new(x, y, z), list);
        }
    }
}

fn ghostly_orbs(time: f32) -> Arc<dyn Hitable> {
    let mut list: List = Vec::new();
    let light = material::diffuse_light(5.0, 5.0, 5.0);

    // Main Light
    XZRect {
        x0: -400.0,
        x1: 400.0,
        z0: -400.0,
        z1: 400.0,
        k: 1500.0,
        material: light.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Floor
    let floor = Cuboid::new(
        Vec3::new(-2000.0, -300.0, -2000.0),
        Vec3::new(2000.0, 0.0, 2000.0),
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
        center: Vec3::new(0.0, 80.0, 0.0),
        radius: GLASS_SPHERE_RADIUS,
        material: material::dielectric(1.1),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Glowing Red Ball inside Glass Sphere
    let red_orbit = Orbit3D::new(
        Vec3::new(-45.0, 60.0, 0.0),
        Vec3::new(0.0, 80.0, 0.0),
        -160.0,
    );
    Sphere {
        center: red_orbit.point_at_time(time),
        radius: 12.0,
        material: material::diffuse_light(0.9, 0.2, 0.2),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Glowing Blue Ball inside Glass Sphere
    let blue_orbit = Orbit3D::new(Vec3::new(45.0, 80.0, 0.0), Vec3::new(0.0, 80.0, 0.0), 150.0);
    Sphere {
        center: blue_orbit.point_at_time(time + 1.0),
        radius: 8.0,
        material: material::diffuse_light(0.2, 0.2, 9.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Glowing Green Ball inside Glass Sphere
    let green_orbit = Orbit3D::new(
        Vec3::new(0.0, 120.0, -45.0),
        Vec3::new(0.0, 80.0, 0.0),
        -140.0,
    );
    Sphere {
        center: green_orbit.point_at_time(time + 2.0),
        radius: 10.0,
        material: material::diffuse_light(0.2, 0.9, 0.2),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Mirror core of sphere
    Sphere {
        center: Vec3::new(0.0, 80.0, 0.0),
        radius: 25.0,
        material: material::metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Background
    Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2300.0,
        material: material::lambertion(0.1, 0.1, 0.2),
    }
    .flip_normals()
    .push_into_list_of_boxed_hitables(&mut list);

    Arc::new(BVHNode::new(list))
}

#[allow(dead_code)]
fn ghostly_orbs_view_1(time: f32) -> Image {
    let nx: i32 = 324;
    let ny: i32 = 324;
    let ns: i32 = 100;

    let camera_orbit = Orbit3D::new(
        Vec3::new(200.0, 90.0, -300.0),
        Vec3::new(0.0, 80.0, 0.0),
        2.0,
    );

    let cam = Camera::new(CameraOpts {
        lookfrom: camera_orbit.point_at_time(time),
        lookat: Vec3::new(0.0, 80.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: nx as f32 / ny as f32,
        focus_dist: 10.0,
        aperture: 0.0,
        vfow: 40.0,
    });

    Image {
        width: nx,
        height: ny,
        samples: ns,
        cam,
        world: ghostly_orbs(time),
    }
}

#[allow(dead_code)]
fn ghostly_orbs_view_2(time: f32) -> Image {
    let nx: i32 = 324;
    let ny: i32 = 334;
    let ns: i32 = 2;

    let camera_move = MoveL::new(
        Vec3::new(0.0, 90.0, -300.0),
        Vec3::new(0.0, 300.0, -700.0),
        75.0,
    );

    let cam = Camera::new(CameraOpts {
        lookfrom: camera_move.point_at_time(time),
        lookat: Vec3::new(0.0, 80.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: nx as f32 / ny as f32,
        focus_dist: 10.0,
        aperture: 0.0,
        vfow: 40.0,
    });

    Image {
        width: nx,
        height: ny,
        samples: ns,
        cam,
        world: ghostly_orbs(time),
    }
}

fn main() {
    let renderer = AnimationRenderer::from(Animation {
        fps: 24.0,
        start_time: 0.0,
        end_time: 8.0,
        image_fn: &ghostly_orbs_view_2,
    });
    renderer.render("ghostly_orbs");
}
