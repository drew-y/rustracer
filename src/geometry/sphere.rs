use super::super::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    material::Material,
    ray::Ray,
    vec3::{dot, Vec3},
};
use super::translation::Translation;
use std::f32::consts::PI;
use std::ops::Deref;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / PI;
        (u, v)
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = dot(&r.direction, &r.direction);
        let b = dot(&oc, &r.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let test = |t: f32| t < t_max && t > t_min;
            let gen_hit_record = |t: f32| {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = Self::get_sphere_uv(&normal);
                Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    material: &self.material,
                    normal,
                })
            };

            let mut t = (-b - (b * b - a * c).sqrt()) / a;
            if test(t) {
                return gen_hit_record(t);
            };
            t = (-b + (b * b - a * c).sqrt()) / a;
            if test(t) {
                return gen_hit_record(t);
            };
        };
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

impl Hitable for Box<Sphere> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.deref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.deref().bounding_box()
    }
}

impl Translation for Sphere {}
