use super::texture::Texture;
use crate::tracer::Vec3;
use std::ops::Deref;

#[derive(Clone)]
pub struct ImageTexture {
    pub image: Vec<u8>,
    pub nx: u32,
    pub ny: u32,
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let nx = self.nx as usize;
        let ny = self.ny as usize;
        let i = ((u * nx as f32) as usize).min(nx - 1).max(0);
        let j = (((1.0 - v) * ny as f32) as usize).min(ny - 1).max(0);
        let index = 3 * i + 3 * nx * j;
        let r = self.image[index] as f32 / 255.0;
        let g = self.image[index + 1] as f32 / 255.0;
        let b = self.image[index + 2] as f32 / 255.0;
        Vec3::new(r, g, b)
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new(self.deref().clone())
    }
}