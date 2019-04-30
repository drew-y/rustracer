use crate::tracer::Vec3;
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
