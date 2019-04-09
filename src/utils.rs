use rand::prelude::*;
use super::vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f32>();
    let mut sample = || 2.0 * Vec3::new(rnd(), rnd(), rnd()) - Vec3::new(1.0, 1.0, 1.0);
    let mut p = sample();
    while p.squared_length() >= 1.0 { p = sample(); }
    p
}
