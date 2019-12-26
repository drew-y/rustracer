use super::{camera::Camera, hitable::Hitable};
use std::sync::Arc;

#[derive(Clone)]
pub struct Scene {
    pub nx: i32,
    pub ny: i32,
    pub ns: i32,
    pub cam: Camera,
    pub world: Arc<dyn Hitable>,
}

#[derive(Clone)]
pub struct AnimatedScene {
    pub fps: f32,
    pub start: f32,
    pub end: f32,
    /// A function that returns a scene when passed time in seconds
    pub scene_fn: &'static dyn Fn(f32) -> Scene,
}
