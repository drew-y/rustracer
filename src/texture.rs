use super::vec3::Vec3;
use std::ops::Deref;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

impl<T: Texture> Texture for Box<T> {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.deref().value(u, v, p)
    }
}

impl<T: Texture> Texture for Arc<T> {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.deref().value(u, v, p)
    }
}

pub struct ConstantTexture {
    pub color: Vec3
}

impl ConstantTexture {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        ConstantTexture {
            color: Vec3::new(r, g, b)
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Arc<Texture>,
    pub even: Arc<Texture>
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
