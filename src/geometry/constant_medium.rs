use super::super::{material::Material, tracer::*};
use super::translation::Translation;
use rand::prelude::*;
use std::f32::MAX;

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundry: BoxHitable,
    pub density: f32,
    pub phase_function: Material,
}

impl Hitable for ConstantMedium {
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
                u: 0.0,
                v: 0.0,
                p: r.point_at_parameter(t),
                normal: Vec3::new(1.0, 0.0, 0.0),
                material: &self.phase_function,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        self.boundry.bounding_box()
    }

    fn box_clone(&self) -> Box<dyn Hitable> {
        Box::new(ConstantMedium {
            boundry: self.boundry.clone(),
            density: self.density,
            phase_function: self.phase_function.clone(),
        })
    }
}

impl Translation for ConstantMedium {}
