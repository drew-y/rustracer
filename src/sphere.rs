use super::hitable::{ Hitable, HitRecord };
use super::vec3::{ Vec3, dot, };
use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
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
                    t, p,
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
