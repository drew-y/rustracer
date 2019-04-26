use super::ray::Ray;
use super::vec3::Vec3;
use rand::prelude::*;
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    w: Vec3,
    u: Vec3,
    v: Vec3,
}

pub struct CameraOpts {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfow: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32,
}

impl Camera {
    fn ranom_in_unit_disk() -> Vec3 {
        let mut rng = thread_rng();
        let mut rnd = || rng.gen::<f32>();
        let mut sample = || 2.0 * Vec3::new(rnd(), rnd(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        let mut p = sample();
        while p.dot(&p) >= 1.0 {
            p = sample()
        }
        p
    }

    pub fn new(
        CameraOpts {
            lookfrom,
            lookat,
            vup,
            vfow,
            aspect,
            aperture,
            focus_dist,
        }: CameraOpts,
    ) -> Camera {
        let theta = vfow * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        Camera {
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * u * focus_dist,
            vertical: 2.0 * half_height * v * focus_dist,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            w,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Camera::ranom_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
