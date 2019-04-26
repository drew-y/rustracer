use image;

/// Returns a tuple with the image data, and its nx, ny values
pub fn read_image(path: String) -> (Vec<u8>, u32, u32) {
    let pic = image::open(path).expect("Image not found").to_rgb();
    let (nx, ny) = pic.dimensions();
    let data = pic.into_raw();
    (data, nx, ny)
}
