use std::f64::consts::PI;
use rand::prelude::*;
use super::vec3::{ Vec3, cross, unit_vector, dot };
use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
    w: Vec3,
    u: Vec3,
    v: Vec3
}

pub struct CameraOpts {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfow: f64,
    pub aspect: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub time0: f64,
    pub time1: f64
}

impl Camera {
    fn ranom_in_unit_disk() -> Vec3 {
        let mut rng = thread_rng();
        let mut rnd = || rng.gen::<f64>();
        let mut sample = || 2.0 * Vec3::new(rnd(), rnd(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        let mut p = sample();
        while dot(&p, &p) >= 1.0 { p = sample() };
        p
    }

    pub fn new(CameraOpts {
        lookfrom, lookat, vup, vfow, aspect, aperture, focus_dist,
        time0, time1
    }: CameraOpts) -> Camera {
        let theta = vfow * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        Camera {
            lower_left_corner: lookfrom - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * u * focus_dist,
            vertical: 2.0 * half_height * v * focus_dist,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            w, u, v, time0, time1
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = thread_rng();
        let rd = self.lens_radius * Camera::ranom_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + rng.gen::<f64>() * (self.time1 - self.time0);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal +
                t * self.vertical - self.origin - offset,
            time
        }
    }
}
