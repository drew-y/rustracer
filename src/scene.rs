use super::{
    geometry::{
        box_geo::BoxGeo,
        rect::{XYRect, XZRect, YZRect},
        sphere::Sphere,
        translation::Translation,
    },
    hitable::Hitable,
    material::{
        self, dielectric, diffuse_light, lambertion,
        Material::{Lambertion, Metal},
    },
    texture::{CheckerTexture, ConstantTexture},
    vec3::Vec3,
};
use rand::prelude::*;

#[allow(dead_code)]
pub fn random_scene() -> Vec<Box<Hitable>> {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f32>();
    let fl = |i: i32| i as f32;
    let mut list: Vec<Box<Hitable>> = vec![];

    // Generate random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rnd();
            let center = Vec3::new(fl(a) + 0.9 * rnd(), 0.2, fl(b) + 0.9 * rnd());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
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
            };
        }
    }

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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white.clone(),
    )
    .push_into_list_of_boxed_hitables(&mut list);

    BoxGeo::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white.clone(),
    )
    .push_into_list_of_boxed_hitables(&mut list);

    list
}
