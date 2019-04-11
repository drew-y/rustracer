use super::super::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
};
use rand::prelude::*;
use std::cmp::Ordering;

pub struct BVHNode {
    pub left: Option<Box<Hitable>>,
    pub right: Option<Box<Hitable>>,
    pub bbox: Option<AABB>,
}

impl BVHNode {
    pub fn new(l: Vec<Box<Hitable>>) -> BVHNode {
        let mut list = l;
        let mut rng = thread_rng();
        let axis = (3.0 * rng.gen::<f32>()) as i32;

        match axis {
            0 => list.sort_by(|a, b| Self::box_x_compare(&a, &b)),
            1 => list.sort_by(|a, b| Self::box_y_compare(&a, &b)),
            2 => list.sort_by(|a, b| Self::box_z_compare(&a, &b)),
            _ => panic!("Invalid axis"),
        };

        if list.len() == 1 {
            let left = list.remove(0);
            let bbox = left.bounding_box();
            return BVHNode {
                left: Some(left),
                right: None,
                bbox,
            };
        };

        if list.len() == 2 {
            let left = list.remove(0);
            let right = list.remove(0);
            let box_left = left.bounding_box();
            let box_right = right.bounding_box();

            let mut bbox: Option<AABB> = None;
            if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
                bbox = Some(AABB::surrounding_box(&hit_left, &hit_right));
            };

            return BVHNode {
                left: Some(left),
                right: Some(right),
                bbox,
            };
        };

        let left_list = list.split_off(list.len() / 2);
        let left = Self::new(left_list);
        let right = Self::new(list);
        let box_left = left.bounding_box();
        let box_right = right.bounding_box();

        let mut bbox: Option<AABB> = None;
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            bbox = Some(AABB::surrounding_box(&hit_left, &hit_right));
        };

        BVHNode {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            bbox,
        }
    }

    fn box_x_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
        let box_left = a.bounding_box();
        let box_right = b.bounding_box();
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            if let Some(cmp) = hit_left.min.x.partial_cmp(&hit_right.min.x) {
                return cmp;
            }
        };

        panic!("Error in BVH bounding box gen");
    }

    fn box_y_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
        let box_left = a.bounding_box();
        let box_right = b.bounding_box();
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            if let Some(cmp) = hit_left.min.y.partial_cmp(&hit_right.min.y) {
                return cmp;
            }
        };

        panic!("Error in BVH bounding box gen");
    }

    fn box_z_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
        let box_left = a.bounding_box();
        let box_right = b.bounding_box();
        if let (Some(hit_left), Some(hit_right)) = (box_left, box_right) {
            if let Some(cmp) = hit_left.min.z.partial_cmp(&hit_right.min.z) {
                return cmp;
            }
        };

        panic!("Error in BVH bounding box gen");
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.is_none() {
            return None;
        };
        if let Some((new_t_min, new_t_max)) = self.bbox.unwrap().hit(r, t_min, t_max) {
            let hit_left = if let Some(left) = &self.left {
                left.hit(r, new_t_min, new_t_max)
            } else {
                None
            };

            let hit_right = if let Some(right) = &self.right {
                right.hit(r, new_t_min, new_t_max)
            } else {
                None
            };

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
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.bbox
    }
}
