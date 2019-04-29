use super::perlin::Perlin;
use super::tracer::Vec3;
use std::ops::Deref;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
    fn box_clone(&self) -> Box<Texture>;
}

impl<T: Texture> Texture for Box<T> {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.deref().value(u, v, p)
    }

    fn box_clone(&self) -> Box<Texture> {
        self.deref().box_clone()
    }
}

impl Clone for Box<Texture> {
    fn clone(&self) -> Box<Texture> {
        self.box_clone()
    }
}

impl<T: Texture> Texture for Arc<T> {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.deref().value(u, v, p)
    }

    fn box_clone(&self) -> Box<Texture> {
        self.deref().box_clone()
    }
}

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

    fn box_clone(&self) -> Box<Texture> {
        Box::new(self.deref().clone())
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    pub odd: Box<Texture>,
    pub even: Box<Texture>,
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

    fn box_clone(&self) -> Box<Texture> {
        Box::new(self.deref().clone())
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    scale: f32,
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * 3.0 * self.noise.turb(&(self.scale * p), 7).sin()
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new(self.deref().clone())
    }
}

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
