use super::super::{
    material::Material,
    hitable::{ Hitable, HitRecord },
    aabb::AABB,
    vec3::Vec3,
    ray::Ray,
};

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max { return None };
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None
        };
        // let u = (x - self.x0) / (self.x1 - self.x0); // TODO (Use this)
        // let u = (x - self.x0) / (self.x1 - self.x0); // TODO (Use this)
        Some(HitRecord {
            t,
            p: r.point_at_parameter(t),
            material: &self.material,
            normal: Vec3::new(0.0, 0.0, 1.0)
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001)
        })
    }
}
