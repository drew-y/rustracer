mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;

use vec3::{ Vec3, unit_vector };
use hitable::{ Hitable, HitableList };
use ray::Ray;
use std::f64::MAX;
use std::thread;
use std::sync::Arc;
use sphere::{ Sphere, MovingSphere };
use camera::{ Camera, CameraOpts };
use rand::prelude::*;
use material::Material::{ Lambertion, Metal, Dielectric };

fn color<T: Hitable>(r: &Ray, world: &T, depth: i64) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, MAX) {
        if depth >= 50 { return Vec3::new(0.0, 0.0, 0.0); }
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * color(&scattered, world, depth + 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn random_scene() -> Arc<Hitable> {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f64>();
    let fl = |i: i32| f64::from(i);
    let mut list = HitableList::<Box<Hitable>>::new();

    // Generate random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rnd();
            let center = Vec3::new(fl(a) + 0.9 * rnd(), 0.2, fl(b) + 0.9 * rnd());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // Diffuse
                if choose_mat < 0.8 {
                    list.push(Box::new(MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0.0, 0.5 * rnd(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Lambertion { albedo: Vec3::new(rnd() * rnd(), rnd() * rnd(), rnd() * rnd()) }
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

    // Floor
    list.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertion { albedo: Vec3::new(0.5, 0.5, 0.5) }
    }));

    // Big sphere trio
    list.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0,
        material: Dielectric { ref_idx: 1.5 }
    }));

    list.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0,
        material: Lambertion { albedo: Vec3::new(0.4, 0.2, 0.1) }
    }));

    list.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0,
        material: Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0
        }
    }));

    Arc::new(list)
}

struct Scene<'a> {
    starty: i32,
    endy: i32,
    nx: i32,
    ny: i32,
    ns: i32,
    cam: &'a Camera,
    hitable: Arc<Hitable>
}

fn render<T: Hitable>(scene: Scene) -> Vec<String> {
    let mut file: Vec<String> = vec![];
    let Scene { ns, nx, ny, cam, hitable, starty, endy } = scene;
    let mut rng = thread_rng();

    for j in (starty..endy).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(nx);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(ny);
                let r = cam.get_ray(u, v);
                col += color(&r, &hitable, 0);
            };

            col /= f64::from(ns);
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();
            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;
            file.push(format!("{} {} {}\n", ir, ig, ib))
        }
    }
    file
}

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    let mut file = vec![format!("P3\n{} {}\n255\n", nx, ny)];

    let world = random_scene();

    let cam = Camera::new(CameraOpts {
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        aspect: f64::from(nx) / f64::from(ny),
        focus_dist: 10.0, aperture: 0.1, vfow: 20.0, time0: 0.0, time1: 1.0
    });

    let mut render_threads: Vec<thread::JoinHandle<Vec<String>>> = vec![];
    let thread_count = 8;
    let y_section_size = ny / thread_count;
    let mut starty = ny - y_section_size;
    let mut endy = ny;
    for _render_thread_num in 0..thread_count {
        let thread_world = world.clone();
        let render_thread = thread::spawn(move || render::<Arc<Hitable>>(Scene {
            nx, ny, ns, starty, endy,
            cam: &cam,
            hitable: thread_world
        }));
        render_threads.push(render_thread);
        endy = starty;
        starty -= y_section_size;
    }

    for render_thread in render_threads {
        file.extend(render_thread.join().unwrap());
    }

    for string in file {
        println!("{}", string);
    }
}
