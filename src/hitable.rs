use super::vec3::Vec3;
use super::ray::Ray;
use super::material::Material;
use super::aabb::AABB;
use std::sync::Arc;
use std::ops::Deref;

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    /// If t0 and t1 inside the hitable's box it will return (tmin, tmax)
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material
}

pub struct HitableList<T: Hitable> {
    list: Vec<T>
}

impl<T: Hitable> HitableList<T> {
    pub fn new() -> HitableList<T> {
        HitableList { list: Vec::new() }
    }

    pub fn push(&mut self, v: T) {
        self.list.push(v);
    }
}

impl<T: Hitable> Hitable for HitableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far_t = t_max;
        let mut closest_so_far: Option<HitRecord> = None;
        for hitable in &self.list {
            if let Some(hit) = hitable.hit(&r, t_min, closest_so_far_t) {
                closest_so_far_t = hit.t;
                closest_so_far = Some(hit);
            }
        };
        closest_so_far
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.list.len() < 1 { return None; };

        let first_box = self.list[0].bounding_box(t0, t1);

        let mut current_box: AABB;
        if let Some(bbox) = first_box {
            current_box = bbox;
        } else {
            return None;
        }

        for hitable in &self.list[1..] {
            if let Some(bbox) = hitable.bounding_box(t0, t1) {
                current_box = bbox;
            } else {
                return None;
            }
        };

        Some(current_box)
    }
}

impl Hitable for Arc<Hitable> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.deref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.deref().bounding_box(t0, t1)
    }
}

impl Hitable for Box<Hitable> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.deref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.deref().bounding_box(t0, t1)
    }
}
