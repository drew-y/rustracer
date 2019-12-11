use super::super::{material::Material, tracer::*};
use super::translation::Translation;

#[derive(Clone)]
pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        };
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        };
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        Some(HitRecord {
            t,
            u,
            v,
            p: r.point_at_parameter(t),
            material: &self.material,
            normal: Vec3::new(0.0, 0.0, 1.0),
        })
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }

    fn box_clone(&self) -> BoxHitable {
        Box::new(self.clone())
    }
}

impl Translation for XYRect {}

#[derive(Clone)]
pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hitable for XZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return None;
        };
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        };
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        Some(HitRecord {
            t,
            u,
            v,
            p: r.point_at_parameter(t),
            material: &self.material,
            normal: Vec3::new(0.0, 1.0, 0.0),
        })
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox {
            min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }

    fn box_clone(&self) -> BoxHitable {
        Box::new(self.clone())
    }
}

impl Translation for XZRect {}

#[derive(Clone)]
pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hitable for YZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t_min || t > t_max {
            return None;
        };
        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        };

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord {
            t,
            u,
            v,
            p: r.point_at_parameter(t),
            material: &self.material,
            normal: Vec3::new(1.0, 0.0, 0.0),
        })
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox {
            min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            max: Vec3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }

    fn box_clone(&self) -> BoxHitable {
        Box::new(self.clone())
    }
}

impl Translation for YZRect {}
