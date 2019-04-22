use super::vec3::Vec3;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Perlin {
    rand_float: [f32; 256],
    x: [i32; 256],
    y: [i32; 256],
    z: [i32; 256],
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            rand_float: Self::gen_float_list(),
            x: Self::gen_int_list(),
            y: Self::gen_int_list(),
            z: Self::gen_int_list(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        // let u = p.x - p.x.floor();
        // let v = p.y - p.y.floor();
        // let w = p.z - p.z.floor();
        let i = (4.0 * p.x) as usize & 255;
        let j = (4.0 * p.y) as usize & 255;
        let k = (4.0 * p.z) as usize & 255;
        self.rand_float[(self.x[i] ^ self.y[j] ^ self.z[k]) as usize]
    }

    fn gen_float_list() -> [f32; 256] {
        let mut p: [f32; 256] = [0.0; 256];
        let mut rng = thread_rng();
        for i in 0..256 {
            p[i] = rng.gen::<f32>();
        }
        p
    }

    fn permute(p: &mut [i32; 256]) {
        let mut rng = thread_rng();
        for i in 0..p.len() {
            let target = (rng.gen::<f32>() * i as f32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn gen_int_list() -> [i32; 256] {
        let mut p: [i32; 256] = [0; 256];
        for i in 0..256 {
            p[i] = i as i32;
        }
        Self::permute(&mut p);
        p
    }
}
