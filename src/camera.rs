use std::default::Default;
use super::vec3::Vec3;
use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3
}

impl Camera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
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
            vertical: Vec3::new(0.0, 2.0, 1.0),
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}
