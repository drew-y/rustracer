#[allow(dead_code)]
mod animation;
mod geometry;
mod material;
#[allow(dead_code)]
mod scenes;
mod texture;
mod tracer;
mod utils;

use scenes::earth;
use tracer::render;

fn main() {
    let scene = earth();
    render(scene, "test.png");
}
