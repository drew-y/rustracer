#[derive(Clone)]
pub struct Scene {
    pub nx: i32,
    pub ny: i32,
    pub ns: i32,
    pub cam: Camera,
    pub world: Arc<Hitable>,
}
