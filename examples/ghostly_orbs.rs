extern crate rustracer;

use rustracer::animation::*;
use rustracer::geometry::*;
use rustracer::material::{self};
use rustracer::tracer::*;
use std::sync::Arc;

type List = Vec<BoxHitable>;
static MAIN_SPHERE_RADIUS: f32 = 65.0;

fn cube(center: Vec3, list: &mut List) {
    Cuboid::cube(5.5, center, material::lambertion(0.05, 0.05, 0.05))
        .push_into_list_of_boxed_hitables(list);
}

fn cube_grid(list: &mut List, time: f32) {
    for x in -60..60 {
        for z in -60..60 {
            let distance_increment = 26.5;
            let x = x as f32 * distance_increment;
            let z = z as f32 * distance_increment;
            let distance = (x.powi(2) + z.powi(2)).sqrt();

            if distance < MAIN_SPHERE_RADIUS + 50.0 {
                continue;
            }

            let y = (distance * 0.005 - (time + 2.0) * 2.0).cos() * 100.0 - 30.0;
            cube(Vec3::new(x, y, z), list);
        }
    }
}

fn ghostly_orbs(time: f32) -> Arc<dyn Hitable> {
    let mut list: List = Vec::new();
    let light = material::diffuse_light(4.0, 4.0, 4.0);
    let metal = material::metal(Vec3::new(0.6, 0.6, 0.6), 0.0);

    // Main Light
    XZRect {
        x0: -600.0,
        x1: 600.0,
        z0: -600.0,
        z1: 600.0,
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
        phase_function: material::isotropic(0.99, 0.99, 0.99),
    }
    .push_into_list_of_boxed_hitables(&mut list);
    floor.push_into_list_of_boxed_hitables(&mut list);

    cube_grid(&mut list, time);

    // Main Sphere
    Sphere {
        center: Vec3::new(0.0, 80.0, 0.0),
        radius: MAIN_SPHERE_RADIUS,
        material: metal.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let orbit2 = Orbit3D::new(
        Vec3::new(MAIN_SPHERE_RADIUS + 40.0, 85.0, 0.0),
        Vec3::new(0.0, 80.0, 0.0),
        75.0,
    );
    Sphere {
        center: orbit2.point_at_time(time + 1.0),
        radius: 18.0,
        material: metal.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Orbit 1
    let orbit1 = Orbit3D::new(
        Vec3::new(-MAIN_SPHERE_RADIUS - 60.0, 60.0, 0.0),
        Vec3::new(0.0, 80.0, 0.0),
        -110.0,
    );
    Sphere {
        center: orbit1.point_at_time(time),
        radius: 16.0,
        material: metal.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let orbit3 = Orbit3D::new(
        Vec3::new(0.0, 126.0, MAIN_SPHERE_RADIUS + 80.0),
        Vec3::new(0.0, 80.0, 0.0),
        -110.0,
    );
    Sphere {
        center: orbit3.point_at_time(time + 2.0),
        radius: 10.0,
        material: metal.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Background
    Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2300.0,
        material: material::lambertion(1.0, 1.0, 1.0),
    }
    .flip_normals()
    .push_into_list_of_boxed_hitables(&mut list);

    Arc::new(BVHNode::new(list))
}

#[allow(dead_code)]
fn ghostly_orbs_view_1(time: f32) -> Image {
    let nx: i32 = 406;
    let ny: i32 = 406;
    let ns: i32 = 250;

    let camera_orbit = Orbit3D::new(
        Vec3::new(200.0, 600.0, -800.0),
        Vec3::new(0.0, 600.0, 0.0),
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
    let nx: i32 = 406;
    let ny: i32 = 406;
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
        image_fn: &ghostly_orbs_view_1,
    });
    renderer.render_with_progress_bar("ghostly_orbs");
}
