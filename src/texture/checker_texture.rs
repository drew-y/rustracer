use super::texture::Texture;
use crate::tracer::Vec3;
use std::ops::Deref;

#[derive(Clone)]
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self.deref().clone())
    }
}
