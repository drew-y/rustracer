use std::cmp::Ordering;
use rand::prelude::*;
use super::hitable::{ Hitable, HitRecord };
use super::ray::Ray;
use super::aabb::AABB;
use std::sync::Arc;

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
