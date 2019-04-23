use super::vec3::Vec3;
use rand::{prelude::*, seq::SliceRandom};

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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor();
        let j = p.y.floor();
        let k = p.z.floor();
        let mut c: [[[f32; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_index = ((i as i32 + di as i32) & 255) as usize;
                    let y_index = ((j as i32 + dj as i32) & 255) as usize;
                    let z_index = ((k as i32 + dk as i32) & 255) as usize;
                    let rand_float_index =
                        (self.x[x_index] ^ self.y[y_index] ^ self.z[z_index]) as usize;
                    c[di][dj][dk] = self.rand_float[rand_float_index];
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f32 * u + (1.0 - i as f32) * (1.0 - u))
                        * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                        * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
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
        p.shuffle(&mut rng);
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
