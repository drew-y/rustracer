use super::vec3::Vec3;
use super::ray::Ray;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

pub struct HitableList<T: Hitable> {
    pub list: Vec<T>
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
}
