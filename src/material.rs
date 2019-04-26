use super::hitable::HitRecord;
use super::ray::Ray;
use super::texture::{ConstantTexture, ImageTexture, Texture};
use super::utils::read_image;
use super::vec3::Vec3;
use rand::prelude::*;

#[derive(Clone)]
pub enum Material {
    Lambertion { albedo: Box<Texture> },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ref_idx: f32 },
    DiffuseLight { emit: Box<Texture> },
    Isotropic { albedo: Box<Texture> },
}

impl Material {
    fn lambertion_scatter(_r: &Ray, rec: &HitRecord, albedo: &Box<Texture>) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        return Some((
            albedo.value(rec.u, rec.v, rec.p),
            Ray {
                origin: rec.p,
                direction: target - rec.p,
            },
        ));
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = v.unit_vector();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
        } else {
            None
        }
    }

    fn metal_scatter(r: &Ray, rec: &HitRecord, albedo: &Vec3, fuzz: f32) -> Option<(Vec3, Ray)> {
        let reflected = Material::reflect(&r.direction.unit_vector(), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + fuzz * Vec3::random_in_unit_sphere(),
        };

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((*albedo, scattered))
        } else {
            None
        }
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 * ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn dielectric_scatter(r: &Ray, rec: &HitRecord, ref_idx: f32) -> Option<(Vec3, Ray)> {
        let reflected = Material::reflect(&r.direction, &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let dir_dot_norm = r.direction.dot(&rec.normal);
        let (outward_normal, ni_over_nt, cosine) = if dir_dot_norm > 0.0 {
            (
                -rec.normal,
                ref_idx,
                ref_idx * dir_dot_norm / r.direction.length(),
            )
        } else {
            (
                rec.normal,
                1.0 / ref_idx,
                -dir_dot_norm / r.direction.length(),
            )
        };

        let mut rng = thread_rng();
        if let Some(refracted) = Material::refract(&r.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = Material::schlick(cosine, ref_idx);
            if rng.gen::<f32>() >= reflect_prob {
                return Some((
                    attenuation,
                    Ray {
                        origin: rec.p,
                        direction: refracted,
                    },
                ));
            }
        }

        Some((
            attenuation,
            Ray {
                origin: rec.p,
                direction: reflected,
            },
        ))
    }

    fn isotropic_scatter(_r: &Ray, rec: &HitRecord, albedo: &Box<Texture>) -> Option<(Vec3, Ray)> {
        let scattered = Ray {
            origin: rec.p,
            direction: Vec3::random_in_unit_sphere(),
        };

        let attenuation = albedo.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }

    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertion { albedo } => Material::lambertion_scatter(r, rec, albedo),
            Material::Metal { albedo, fuzz } => Material::metal_scatter(r, rec, albedo, *fuzz),
            Material::Dielectric { ref_idx } => Material::dielectric_scatter(r, rec, *ref_idx),
            Material::DiffuseLight { .. } => None,
            Material::Isotropic { albedo } => Material::isotropic_scatter(r, rec, albedo),
        }
    }

    pub fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        match self {
            Material::DiffuseLight { emit } => emit.value(u, v, p),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

/// Create a basic lambertion material
pub fn lambertion(r: f32, g: f32, b: f32) -> Material {
    Material::Lambertion {
        albedo: Box::new(ConstantTexture::new(r, g, b)),
    }
}

pub fn lambertion_with_image(path: &str) -> Material {
    let image = read_image(path.to_string());
    Material::Lambertion {
        albedo: ImageTexture {
            image: image.0,
            nx: image.1,
            ny: image.2,
        }
        .box_clone(),
    }
}

/// Create a basic metal material
pub fn metal(color: Vec3, fuzz: f32) -> Material {
    Material::Metal {
        albedo: color,
        fuzz,
    }
}
/// Create a basic dielectric material
pub fn dielectric(ref_idx: f32) -> Material {
    Material::Dielectric { ref_idx }
}

/// Create a basic diffuse light material
pub fn diffuse_light(r: f32, g: f32, b: f32) -> Material {
    Material::DiffuseLight {
        emit: Box::new(ConstantTexture::new(r, g, b)),
    }
}

/// Create a basic isotropic material
pub fn isotropic(r: f32, g: f32, b: f32) -> Material {
    Material::Isotropic {
        albedo: Box::new(ConstantTexture::new(r, g, b)),
    }
}
