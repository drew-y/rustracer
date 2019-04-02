use std::default::Default;
use std::f64::consts::PI;
use super::vec3::{ Vec3, cross, unit_vector };
use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfow: f64, aspect: f64) -> Camera {
        let theta = vfow * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        Camera {
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal +
                v * self.vertical - self.origin
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}
