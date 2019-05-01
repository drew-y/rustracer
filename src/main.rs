#[allow(dead_code)]
mod animation;
#[allow(dead_code)]
mod animations;
mod geometry;
mod material;
#[allow(dead_code)]
mod scenes;
mod texture;
mod tracer;
mod utils;

use animations::moon_orbits_earth;

fn main() {
    moon_orbits_earth();
}
