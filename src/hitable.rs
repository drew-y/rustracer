use super::aabb::AABB;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
use std::ops::Deref;
use std::sync::Arc;

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    /// If t0 and t1 inside the hitable's box it will return (tmin, tmax)
    fn bounding_box(&self) -> Option<AABB>;
}

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl Hitable for Arc<Hitable> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.deref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.deref().bounding_box()
    }
}

impl Hitable for Box<Hitable> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.deref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.deref().bounding_box()
    }
}
