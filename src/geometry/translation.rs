use super::super::{
    hitable::{ Hitable, HitRecord },
    aabb::AABB,
    ray::Ray
};

pub struct FlipNormals<T: Hitable> {
    hitable: T
}

impl<T: Hitable> Hitable for FlipNormals<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(rec) = self.hitable.hit(r, t_min, t_max) {
            Some(HitRecord {
                normal: -rec.normal,
                ..rec
            })
        } else { None }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.hitable.bounding_box()
    }
}

pub fn flip_normals<T: Hitable>(hitable: T) -> FlipNormals<T> {
    FlipNormals { hitable: hitable }
}
