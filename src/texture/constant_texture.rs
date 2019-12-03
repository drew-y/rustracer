use super::texture::Texture;
use crate::tracer::Vec3;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        ConstantTexture {
            color: Vec3::new(r, g, b),
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self.deref().clone())
    }
}
