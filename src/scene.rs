use super::{
    geometry::{
        box_geo::BoxGeo,
        constant_medium::ConstantMedium,
        rect::{XYRect, XZRect, YZRect},
        sphere::Sphere,
        translation::Translation,
    },
    hitable::Hitable,
    material::{
        self, dielectric, diffuse_light, isotropic, lambertion,
        Material::{Lambertion, Metal},
    },
    texture::{CheckerTexture, ConstantTexture, NoiseTexture, Texture},
    vec3::Vec3,
};
use rand::prelude::*;

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

    BoxGeo::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )
    .rotate_y(-18.0)
    .shift(130.0, 0.0, 65.0)
    .push_into_list_of_boxed_hitables(&mut list);

    BoxGeo::new(
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

    let b1 = BoxGeo::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )
    .rotate_y(-18.0)
    .shift(130.0, 0.0, 65.0);

    let b2 = BoxGeo::new(
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
    let texture = NoiseTexture::new();

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
