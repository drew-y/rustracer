use super::vec3::Vec3;
use super::ray::Ray;
use super::material::Material;
use std::sync::Arc;
use std::ops::Deref;
use std::cmp::Ordering;
use rand::prelude::*;

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

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn surrounding_box(bbox0: &AABB, bbox1: &AABB) -> AABB {
        let min = Vec3::new(
            bbox0.min.x.min(bbox1.min.x),
            bbox0.min.y.min(bbox1.min.y),
            bbox0.min.z.min(bbox1.min.z)
        );

        let max = Vec3::new(
            bbox0.max.x.max(bbox1.max.x),
            bbox0.max.y.max(bbox1.max.y),
            bbox0.max.z.max(bbox1.max.z)
        );

        AABB { min, max }
    }

    /// If we hit returns a tuple of (tmin, tmax)
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, f64)> {
        let mut new_tmin = tmin;
        let mut new_tmax = tmax;
        for i in 0..3 {
            let inv_d = 1.0 / r.direction.index(i).unwrap();
            let mut t0 = (self.min.index(i).unwrap() - r.origin.index(i).unwrap()) *
                inv_d;
            let mut t1 = (self.max.index(i).unwrap() - r.origin.index(i).unwrap()) *
                inv_d;
            if inv_d < 0.0 { std::mem::swap(&mut t0, & mut t1); }
            new_tmin = if t0 > tmin { t0 } else { tmin };
            new_tmax = if t1 < tmax { t1 } else { tmax };
            if new_tmax <= new_tmin { return None; }
        }
        Some((new_tmin, new_tmax))
    }
}

pub struct BVHNode {
    pub left: Arc<Hitable>,
    pub right: Arc<Hitable>,
    pub bbox: Option<AABB>
}

impl BVHNode {
    fn new(list: &mut [Arc<Hitable>], time0: f64, time1: f64) -> BVHNode {
        let mut rng = thread_rng();
        let axis = (3.0 * rng.gen::<f64>()) as i32;

        match axis {
            0 => list.sort_by(|a, b| Self::box_x_compare(&a, &b)),
            1 => list.sort_by(|a, b| Self::box_y_compare(&a, &b)),
            2 => list.sort_by(|a, b| Self::box_z_compare(&a, &b)),
            _ => panic!("Invalid axis")
        };

        if list.len() == 1 {
            return BVHNode {
                left: list[0].clone(),
                right: list[0].clone(),
                bbox: list[0].bounding_box(time0, time1)
            }
        };

        if list.len() == 2 {
            let left = list[0].clone();
            let right = list[1].clone();
            let box_left = left.bounding_box(time0, time1);
            let box_right = right.bounding_box(time0, time1);

            let mut bbox: Option<AABB> = None;
            if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
                bbox = Some(AABB::surrounding_box(&hit_left, &hit_right));
            };

            return BVHNode {
                left: left.clone(),
                right: right.clone(),
                bbox
            }
        };

        let (left_list, right_list) = list.split_at_mut(list.len() / 2);
        let left = Self::new(left_list, time0, time1);
        let right = Self::new(right_list, time0, time1);
        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        let mut bbox: Option<AABB> = None;
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            bbox = Some(AABB::surrounding_box(&hit_left, &hit_right));
        };

        BVHNode {
            left: Arc::new(left),
            right: Arc::new(right),
            bbox
        }
    }

    fn box_x_compare(a: &Arc<Hitable>, b: &Arc<Hitable>) -> Ordering {
        let box_left = a.bounding_box(0.0, 0.0);
        let box_right = b.bounding_box(0.0, 0.0);
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            if let Some(cmp) = hit_left.min.x.partial_cmp(&hit_right.min.x) {
                return cmp;
            }
        };

        panic!("Error in BVH bounding box gen");
    }

    fn box_y_compare(a: &Arc<Hitable>, b: &Arc<Hitable>) -> Ordering {
        let box_left = a.bounding_box(0.0, 0.0);
        let box_right = b.bounding_box(0.0, 0.0);
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            if let Some(cmp) = hit_left.min.y.partial_cmp(&hit_right.min.y) {
                return cmp;
            }
        };

        panic!("Error in BVH bounding box gen");
    }

    fn box_z_compare(a: &Arc<Hitable>, b: &Arc<Hitable>) -> Ordering {
        let box_left = a.bounding_box(0.0, 0.0);
        let box_right = b.bounding_box(0.0, 0.0);
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            if let Some(cmp) = hit_left.min.z.partial_cmp(&hit_right.min.z) {
                return cmp;
            }
        };

        panic!("Error in BVH bounding box gen");
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.is_none() { return None };
        if let Some((new_t_min, new_t_max)) = self.bbox.unwrap().hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, new_t_min, new_t_max);
            let hit_right = self.right.hit(r, new_t_min, new_t_max);
            if let (Some(left_rec), Some(right_rec)) = (hit_left, hit_right) {
                if left_rec.t < right_rec.t {
                    Some(left_rec)
                } else {
                    Some(right_rec)
                }
            } else if hit_left.is_some() {
                hit_left
            } else {
                hit_right
            }
        } else { None }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        self.bbox
    }
}
