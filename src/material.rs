use super::vec3::{ Vec3, dot, unit_vector };
use super::hitable::HitRecord;
use super::ray::Ray;
use super::sphere::random_in_unit_sphere;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertion { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 }
}

impl Material {
    fn lambertion_scatter(_r: &Ray, rec: &HitRecord, albedo: &Vec3) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return Some((
            *albedo,
            Ray { origin: rec.p, direction: target - rec.p }
        ))
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * dot(v, n) * n
    }

    fn metal_scatter(r: &Ray, rec: &HitRecord, albedo: &Vec3, fuzz: f64) -> Option<(Vec3, Ray)> {
        let reflected = Material::reflect(&unit_vector(r.direction), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + fuzz * random_in_unit_sphere()
        };

        if dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((*albedo, scattered))
        } else { None }
    }

    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertion { albedo } =>
                Material::lambertion_scatter(r, rec, albedo),
            Material::Metal { albedo, fuzz } =>
                Material::metal_scatter(r, rec, albedo, *fuzz)
        }
    }
}
