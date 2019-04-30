use crate::geometry::*;
use crate::material;
use crate::tracer::*;
use std::sync::Arc;

pub fn cornell_box() -> Scene {
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(8);

    let green = material::lambertion(0.12, 0.45, 0.15);
    let red = material::lambertion(0.65, 0.05, 0.05);
    let light = material::diffuse_light(15.0, 15.0, 15.0);
    let white = material::lambertion(0.73, 0.73, 0.73);

    YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: green.clone(),
    }
    .flip_normals()
    .push_into_list_of_boxed_hitables(&mut list);

    YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: red.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        material: light.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: white.clone(),
    }
    .flip_normals()
    .push_into_list_of_boxed_hitables(&mut list);

    XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: white.clone(),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: white.clone(),
    }
    .flip_normals()
    .push_into_list_of_boxed_hitables(&mut list);

    Cuboid::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )
    .rotate_y(-18.0)
    .shift(130.0, 0.0, 65.0)
    .push_into_list_of_boxed_hitables(&mut list);

    Cuboid::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )
    .rotate_y(15.0)
    .shift(265.0, 0.0, 295.0)
    .push_into_list_of_boxed_hitables(&mut list);

    // Big sphere trio
    Sphere {
        center: Vec3::new(350.0, 400.0, 295.0),
        radius: 50.0,
        material: material::dielectric(1.5),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let world = Arc::new(BVHNode::new(list));

    let nx: i32 = 800;
    let ny: i32 = 800;
    let ns: i32 = 7000;
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
