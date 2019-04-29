mod geometry;
mod material;
mod perlin;
#[allow(dead_code)]
mod scenes;
mod texture;
mod tracer;
mod utils;

use scenes::earth;
use tracer::render;

fn main() {
    let scene = earth();
    render(scene);
}
