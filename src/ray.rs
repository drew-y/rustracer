use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
