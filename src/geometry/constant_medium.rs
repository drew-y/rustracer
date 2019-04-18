use super::super::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};
use super::translation::Translation;
use rand::prelude::*;
use std::f32::MAX;

pub struct ConstantMedium<T: Hitable> {
    pub boundry: T,
    pub density: f32,
    pub phase_function: Material,
}

impl<T: Hitable> Hitable for ConstantMedium<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec1 = self.boundry.hit(r, -MAX, MAX)?;
        let mut rec2 = self.boundry.hit(r, rec1.t + 0.0001, MAX)?;
        if rec1.t < t_min {
            rec1.t = t_min
        };
        if rec2.t > t_max {
            rec2.t = t_max;
        };
        if rec1.t >= rec2.t {
            return None;
        };
        if rec1.t < 0.0 {
            rec1.t = 0.0
        };
        let rval = thread_rng().gen::<f32>();
        let distance_inside_boundry = (rec2.t - rec1.t) * r.direction.length();
        let hit_distance = -(1.0 / self.density) * rval.ln();
        if hit_distance < distance_inside_boundry {
            let t = rec1.t + hit_distance / r.direction.length();
            Some(HitRecord {
                t,
                p: r.point_at_parameter(t),
                normal: Vec3::new(1.0, 0.0, 0.0),
                material: &self.phase_function,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundry.bounding_box()
    }
}

impl<T: Hitable> Translation for ConstantMedium<T> {}
