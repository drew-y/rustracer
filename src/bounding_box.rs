use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn surrounding_box(bbox0: &BoundingBox, bbox1: &BoundingBox) -> BoundingBox {
        let min = Vec3::new(
            bbox0.min.x.min(bbox1.min.x),
            bbox0.min.y.min(bbox1.min.y),
            bbox0.min.z.min(bbox1.min.z),
        );

        let max = Vec3::new(
            bbox0.max.x.max(bbox1.max.x),
            bbox0.max.y.max(bbox1.max.y),
            bbox0.max.z.max(bbox1.max.z),
        );

        BoundingBox { min, max }
    }

    /// If we hit returns a tuple of (tmin, tmax)
    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<(f32, f32)> {
        let mut new_tmin = tmin;
        let mut new_tmax = tmax;
        for i in 0..3 {
            let inv_d = 1.0 / r.direction.index(i);
            let mut t0 = (self.min.index(i) - r.origin.index(i)) * inv_d;
            let mut t1 = (self.max.index(i) - r.origin.index(i)) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            new_tmin = if t0 > tmin { t0 } else { tmin };
            new_tmax = if t1 < tmax { t1 } else { tmax };
            if new_tmax <= new_tmin {
                return None;
            }
        }
        Some((new_tmin, new_tmax))
    }
}
