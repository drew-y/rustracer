use super::hitable::{ Hitable, HitRecord, AABB };
use super::vec3::{ Vec3, dot, };
use super::ray::Ray;
use super::material::Material;
use std::ops::Deref;

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

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius)
        })
    }
}

impl Hitable for Box<Sphere> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.deref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.deref().bounding_box(t0, t1)
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material
}

impl MovingSphere {
    fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.time0) /
            (self.time1 - self.time0) * (self.center1 - self.center0))
    }

    fn bounding_box_at_time(&self, time: f64, t0: f64, t1: f64) -> AABB {
        AABB {
            min: self.center(time) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(time) + Vec3::new(self.radius, self.radius, self.radius)
        }
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
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
                    normal: (p - self.center(r.time)) / self.radius
                })
            };

            let mut t = (-b - (b * b - a * c).sqrt()) / a;
            if test(t) { return gen_hit_record(t); };
            t = (-b + (b * b - a * c).sqrt()) / a;
            if test(t) { return gen_hit_record(t); };
        };
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let bbox0 = self.bounding_box_at_time(self.time0, t0, t1);
        let bbox1 = self.bounding_box_at_time(self.time1, t0, t1);
        Some(AABB::surrounding_box(&bbox0, &bbox1))
    }
}

impl Hitable for Box<MovingSphere> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.deref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.deref().bounding_box(t0, t1)
    }
}
