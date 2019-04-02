use super::hitable::{ Hitable, HitRecord };
use super::vec3::{ Vec3, dot, };
use super::ray::Ray;
use rand::prelude::*;
use super::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = dot(&r.direction, &r.direction);
        let b = dot(&oc, &r.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let test = |t: f64| t < t_max && t > t_min;
            let gen_hit_record = |t: f64| {
                let p = r.point_at_parameter(t);
                Some(HitRecord {
                    t, p, material: &self.material,
                    normal: (p - self.center) / self.radius
                })
            };

            let mut t = (-b - (b * b - a * c).sqrt()) / a;
            if test(t) { return gen_hit_record(t); };
            t = (-b + (b * b - a * c).sqrt()) / a;
            if test(t) { return gen_hit_record(t); };
        };
        None
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f64>();
    let mut sample = || 2.0 * Vec3::new(rnd(), rnd(), rnd()) - Vec3::new(1.0, 1.0, 1.0);
    let mut p = sample();
    while p.squared_length() >= 1.0 { p = sample(); }
    p
}
