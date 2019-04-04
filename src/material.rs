use super::vec3::{ Vec3, dot, unit_vector };
use super::hitable::HitRecord;
use super::ray::Ray;
use super::sphere::random_in_unit_sphere;
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertion { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ref_idx: f64 }
}

impl Material {
    fn lambertion_scatter(r: &Ray, rec: &HitRecord, albedo: &Vec3) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return Some((
            *albedo,
            Ray { origin: rec.p, direction: target - rec.p, time: r.time }
        ))
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * dot(v, n) * n
    }

    fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
        let uv = unit_vector(v);
        let dt = dot(&uv, n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * ( 1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
        } else { None }
    }

    fn metal_scatter(r: &Ray, rec: &HitRecord, albedo: &Vec3, fuzz: f64) -> Option<(Vec3, Ray)> {
        let reflected = Material::reflect(&unit_vector(&r.direction), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + fuzz * random_in_unit_sphere(),
            time: r.time
        };

        if dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((*albedo, scattered))
        } else { None }
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 * ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn dielectric_scatter(r: &Ray, rec: &HitRecord, ref_idx: f64) -> Option<(Vec3, Ray)> {
        let reflected = Material::reflect(&r.direction, &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if dot(&r.direction, &rec.normal) > 0.0 {
            (-rec.normal, ref_idx,
                ref_idx * dot(&r.direction, &rec.normal) / r.direction.length())
        } else {
            (rec.normal, 1.0 / ref_idx,
                -dot(&r.direction, &rec.normal) / r.direction.length())
        };

        let mut rng = thread_rng();
        if let Some(refracted) = Material::refract(&r.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = Material::schlick(cosine, ref_idx);
            if rng.gen::<f64>() >= reflect_prob {
                return Some((attenuation, Ray {
                    origin: rec.p, direction: refracted, time: r.time
                }))
            }
        }

        Some((attenuation, Ray { origin: rec.p, direction: reflected, time: r.time }))
    }

    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertion { albedo } =>
                Material::lambertion_scatter(r, rec, albedo),
            Material::Metal { albedo, fuzz } =>
                Material::metal_scatter(r, rec, albedo, *fuzz),
            Material::Dielectric { ref_idx } =>
                Material::dielectric_scatter(r, rec, *ref_idx)
        }
    }
}
