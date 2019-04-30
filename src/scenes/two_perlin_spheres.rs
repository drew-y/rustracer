use crate::geometry::*;
use crate::material::Material;
use crate::texture::*;
use crate::tracer::*;
use std::sync::Arc;

pub fn two_perlin_spheres() -> Scene {
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(8);

    let texture = NoiseTexture::new(4.0);

    Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertion {
            albedo: texture.box_clone(),
        },
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Material::Lambertion {
            albedo: texture.box_clone(),
        },
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
