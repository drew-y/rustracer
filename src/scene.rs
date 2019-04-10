use super::hitable::Hitable;
use rand::prelude::*;
use super::vec3::Vec3;
use super::sphere::Sphere;
use super::material::Material::{ Lambertion, Metal, Dielectric, DiffuseLight };
use super::texture::{ CheckerTexture, ConstantTexture };

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
                if choose_mat < 0.3 {
                    list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: DiffuseLight {
                            emit: Box::new(ConstantTexture::new(rnd() * rnd(), rnd() * rnd(), rnd() * rnd()))
                        }
                    }));
                    continue;
                };

                // Diffuse
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertion {
                            albedo: Box::new(ConstantTexture::new(rnd() * rnd(), rnd() * rnd(), rnd() * rnd()))
                        }
                    }));
                    continue;
                };

                if choose_mat < 0.95 { // Metal
                    list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal {
                            albedo: Vec3::new(0.5 * (1.0 + rnd()), 0.5 * (1.0 + rnd()), 0.5 * (1.0 + rnd())),
                            fuzz: 0.5 * rnd()
                        }
                    }));
                    continue;
                };

                // Glass
                list.push(Box::new(Sphere {
                    center, radius: 0.2,
                    material: Dielectric { ref_idx: 1.5 }
                }))
            };
        };
    };

    let floor_texture = Box::new(CheckerTexture {
        odd: Box::new(ConstantTexture::new(0.2, 0.3, 0.1)),
        even: Box::new(ConstantTexture::new(0.9, 0.9, 0.9)),
    });

    // Floor
    list.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertion { albedo: floor_texture }
    }));

    // Big sphere trio
    list.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0,
        material: Dielectric { ref_idx: 1.5 }
    }));

    list.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0,
        material: Lambertion {
            albedo: Box::new(ConstantTexture::new(0.4, 0.2, 0.1))
        }
    }));

    list.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0,
        material: Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0
        }
    }));

    // Light
    list.push(Box::new(Sphere {
        center: Vec3::new(2.0, 2.0, 2.0), radius: 0.5,
        material: DiffuseLight {
            emit: Box::new(ConstantTexture::new(1.0, 1.0, 1.0))
        }
    }));

    list
}
