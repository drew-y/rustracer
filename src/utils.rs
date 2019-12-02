use image;

/// Returns a tuple with the image data, and its nx, ny values
pub fn read_image(path: String) -> (Vec<u8>, u32, u32) {
    let pic = image::open(path).expect("Image not found").to_rgb();
    let (nx, ny) = pic.dimensions();
    let data = pic.into_raw();
    (data, nx, ny)
}

pub trait FloatCmp<T> {
    /// Panics if compared float is not within 0.000001 of self
    fn assert_nearly_eq(&self, f2: T);
}

impl FloatCmp<f32> for f32 {
    fn assert_nearly_eq(&self, f2: f32) {
        let diff = (self - f2).abs();
        if diff > 0.000001 {
            panic!("Float {} not nearly equal to {}.", self, f2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_nearly_eq_f32_doesnt_panic() {
        (3.3 as f32).assert_nearly_eq(3.3);
    }

    #[test]
    #[should_panic]
    fn assert_nearly_eq_f32_panics() {
        (3.3 as f32).assert_nearly_eq(3.334);
    }
}
