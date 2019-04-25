use super::vec3::Vec3;
use image;
use rand::prelude::*;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let mut rnd = || rng.gen::<f32>();
    let mut sample = || 2.0 * Vec3::new(rnd(), rnd(), rnd()) - Vec3::new(1.0, 1.0, 1.0);
    let mut p = sample();
    while p.squared_length() >= 1.0 {
        p = sample();
    }
    p
}

/// Returns a tuple with the image data, and its nx, ny values
pub fn read_image(path: String) -> (Vec<u8>, u32, u32) {
    let pic = image::open(path).expect("Image not found").to_rgb();
    let (nx, ny) = pic.dimensions();
    let data = pic.into_raw();
    (data, nx, ny)
}
