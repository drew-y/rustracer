use super::vec3::{ Vec3, dot, unit_vector };
use super::hitable::HitRecord;
use super::ray::Ray;
use super::sphere::random_in_unit_sphere;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertion {
    pub albedo: Vec3
}

impl Material for Lambertion {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return Some((
            self.albedo,
            Ray { origin: rec.p, direction: target - rec.p }
        ))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    /// A number between 0 and 1
    pub fuzz: f64
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&unit_vector(r.direction), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_in_unit_sphere()
        };

        if dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else { None }
    }
}
