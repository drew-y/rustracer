use super::{
    camera::{Camera, CameraOpts},
    geometry::{
        bvh::BVHNode,
        constant_medium::ConstantMedium,
        cuboid::Cuboid,
        rect::{XYRect, XZRect, YZRect},
        sphere::Sphere,
        translation::Translation,
    },
    hitable::Hitable,
    material::{
        self, dielectric, diffuse_light, isotropic, lambertion, lambertion_with_image,
        Material::{Lambertion, Metal},
    },
    texture::{CheckerTexture, ConstantTexture, NoiseTexture, Texture},
    vec3::Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Scene {
    pub nx: i32,
    pub ny: i32,
    pub ns: i32,
    pub cam: Camera,
    pub world: Arc<Hitable>,
}

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

fn gen_random_spheres() -> Vec<Box<Hitable>> {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f32>();
    let fl = |i: &i32| *i as f32;
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(484);
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
                material: diffuse_light(rnd() * rnd(), rnd() * rnd(), rnd() * rnd()),
            }
            .push_into_list_of_boxed_hitables(&mut list);
            continue;
        };

        // Diffuse
        if choose_mat < 0.8 {
            Sphere {
                center,
                radius: 0.2,
                material: lambertion(
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
                material: Metal {
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
            material: dielectric(1.5),
        }
        .push_into_list_of_boxed_hitables(&mut list);
    }

    list
}

pub fn random_scene() -> Vec<Box<Hitable>> {
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(488);
    list.extend(gen_random_spheres());

    let floor_texture = Box::new(CheckerTexture {
        odd: Box::new(ConstantTexture::new(0.2, 0.3, 0.1)),
        even: Box::new(ConstantTexture::new(0.9, 0.9, 0.9)),
    });

    // Floor
    Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertion {
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

    list
}

pub fn simple_light() -> Vec<Box<Hitable>> {
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(4);

    let floor_texture = Box::new(CheckerTexture {
        odd: Box::new(ConstantTexture::new(0.2, 0.3, 0.1)),
        even: Box::new(ConstantTexture::new(0.9, 0.9, 0.9)),
    });

    // Floor
    Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertion {
            albedo: floor_texture,
        },
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: material::lambertion(0.4, 0.2, 0.1),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(0.0, 7.0, 0.0),
        radius: 2.0,
        material: material::diffuse_light(4.0, 4.0, 4.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    XYRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        material: material::diffuse_light(4.0, 4.0, 4.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    list
}

pub fn cornell_box() -> Vec<Box<Hitable>> {
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

    list
}

pub fn cornell_smoke() -> Vec<Box<Hitable>> {
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

    let b1 = Cuboid::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )
    .rotate_y(-18.0)
    .shift(130.0, 0.0, 65.0);

    let b2 = Cuboid::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )
    .rotate_y(15.0)
    .shift(265.0, 0.0, 295.0);

    ConstantMedium {
        boundry: b1,
        density: 0.01,
        phase_function: isotropic(1.0, 1.0, 1.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    ConstantMedium {
        boundry: b2,
        density: 0.01,
        phase_function: isotropic(0.0, 0.0, 0.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Big sphere trio
    Sphere {
        center: Vec3::new(350.0, 400.0, 295.0),
        radius: 50.0,
        material: material::dielectric(1.5),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    list
}

pub fn two_perlin_spheres() -> Vec<Box<Hitable>> {
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(2);
    let texture = NoiseTexture::new(4.0);

    Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertion {
            albedo: texture.box_clone(),
        },
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertion {
            albedo: texture.box_clone(),
        },
    }
    .push_into_list_of_boxed_hitables(&mut list);

    list
}

pub fn earth() -> Scene {
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(2);

    // The Sun
    Sphere {
        center: Vec3::new(8.0, 9.0, 20.0),
        radius: 6.0,
        material: diffuse_light(10.0, 10.0, 10.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // The Earth
    Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 3.5,
        material: lambertion_with_image("./earthmap.jpg"),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // Earth's atmosphere
    ConstantMedium {
        boundry: Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 3.51,
            material: lambertion(0.0, 0.0, 0.0),
        },
        density: 20.0,
        phase_function: isotropic(0.45, 0.77, 0.9999),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    // The Moon
    Sphere {
        center: Vec3::new(6.0, 2.0, -1.2),
        radius: 0.6,
        material: lambertion_with_image("./moonmap.jpg"),
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

fn rttnw_final_world() -> Arc<Hitable> {
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(429);
    let mut rng = thread_rng();
    let mut rand = || rng.gen::<f32>();
    let white = lambertion(0.73, 0.73, 0.73);
    let ground = lambertion(0.48, 0.83, 0.53);
    let light = diffuse_light(7.0, 7.0, 7.0);

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
        boundry: boundry1.clone(),
        density: 0.2,
        phase_function: isotropic(0.2, 0.4, 0.9),
    }
    .push_into_list_of_boxed_hitables(&mut list);
    boundry1.push_into_list_of_boxed_hitables(&mut list);

    let boundry2 = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5000.0,
        material: material::dielectric(1.5),
    };

    ConstantMedium {
        boundry: boundry2,
        density: 0.0001,
        phase_function: isotropic(1.0, 1.0, 1.0),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: lambertion_with_image("./earthmap.jpg"),
    }
    .push_into_list_of_boxed_hitables(&mut list);

    Sphere {
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Lambertion {
            albedo: NoiseTexture::new(0.1).box_clone(),
        },
    }
    .push_into_list_of_boxed_hitables(&mut list);

    let mut list2: Vec<Box<Hitable>> = Vec::with_capacity(429);
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

pub fn rttnw_final_scene() -> Scene {
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
