extern crate rustracer;

use rand::prelude::*;
use rustracer::geometry::*;
use rustracer::material::{self, Material};
use rustracer::texture::*;
use rustracer::tracer::*;
use std::sync::Arc;

fn gen_coords() -> [(i32, i32); 484] {
    let mut coords: [(i32, i32); 484] = [(0, 0); 484];
    let mut index = 0;
    for a in -11..11 {
        for b in -11..11 {
            coords[index] = (a, b);
            index += 1;
        }
    }
    coords
}

fn gen_random_spheres() -> Vec<Box<dyn Hitable>> {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f32>();
    let fl = |i: &i32| *i as f32;
    let mut list: Vec<Box<dyn Hitable>> = Vec::with_capacity(484);
    let coords = gen_coords();

    // Generate random spheres
    for (a, b) in coords.iter() {
        let choose_mat = rnd();
        let center = Vec3::new(fl(a) + 0.9 * rnd(), 0.2, fl(b) + 0.9 * rnd());

        // Ensure we dont intersect with the main spheres
        if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
            continue;
        };

        // Diffuse Light
        if choose_mat < 0.6 {
            Sphere {
                center,
                radius: 0.2,
                material: material::diffuse_light(rnd() * rnd(), rnd() * rnd(), rnd() * rnd()),
            }
            .push_into_list_of_boxed_hitables(&mut list);
            continue;
        };

        // Diffuse
        if choose_mat < 0.8 {
            Sphere {
                center,
                radius: 0.2,
                material: material::lambertion(
                    rnd() * rnd() * 4.0,
                    rnd() * rnd() * 4.0,
                    rnd() * rnd() * 4.0,
                ),
            }
            .push_into_list_of_boxed_hitables(&mut list);
            continue;
        };

        // Metal
        if choose_mat < 0.95 {
            Sphere {
                center,
                radius: 0.2,
                material: Material::Metal {
                    albedo: Vec3::new(
                        0.5 * (1.0 + rnd()),
                        0.5 * (1.0 + rnd()),
                        0.5 * (1.0 + rnd()),
                    ),
                    fuzz: 0.5 * rnd(),
                },
            }
            .push_into_list_of_boxed_hitables(&mut list);
            continue;
        };

        // Glass
        Sphere {
            center,
            radius: 0.2,
            material: material::dielectric(1.5),
        }
        .push_into_list_of_boxed_hitables(&mut list);
    }

    list
}

pub fn random_spheres() -> Image {
    let mut list: Vec<Box<dyn Hitable>> = Vec::with_capacity(488);
    list.extend(gen_random_spheres());

    let floor_texture = Box::new(CheckerTexture {
        odd: Box::new(ConstantTexture::new(0.2, 0.3, 0.1)),
        even: Box::new(ConstantTexture::new(0.9, 0.9, 0.9)),
    });

    // Floor
    Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertion {
            albedo: floor_texture,
        },
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Big sphere trio
    Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material::dielectric(1.5),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material::lambertion(0.4, 0.2, 0.1),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material::metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let nx: i32 = 400;
    let ny: i32 = 400;
    let ns: i32 = 40;
    let cam = Camera::new(CameraOpts {
        lookfrom: Vec3::new(5.0, 5.0, -10.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
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
        world: Arc::new(BVHNode::new(list)),
    }
}

fn main() {
    let renderer = Renderer::from(random_spheres());
    renderer.render("./random_spheres.png");
}
