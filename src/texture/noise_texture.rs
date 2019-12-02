use super::texture::Texture;
use crate::tracer::Vec3;
use lazy_static;
use rand::{prelude::*, seq::SliceRandom};
use std::ops::Deref;

#[derive(Clone)]
pub struct NoiseTexture {
    scale: f32,
    noise: &'static Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            scale,
            noise: &PERLIN,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * 3.0 * self.noise.turb(&(self.scale * p), 7).sin()
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new(self.deref().clone())
    }
}

#[derive(Copy, Clone)]
struct Perlin {
    rand_vec3: [Vec3; 256],
    x: [i32; 256],
    y: [i32; 256],
    z: [i32; 256],
}

impl Perlin {
    fn new() -> Perlin {
        Perlin {
            rand_vec3: Self::gen_rand_vec3_list(),
            x: Self::gen_rand_int_list(),
            y: Self::gen_rand_int_list(),
            z: Self::gen_rand_int_list(),
        }
    }

    fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor();
        let j = p.y.floor();
        let k = p.z.floor();
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_index = ((i as i32 + di as i32) & 255) as usize;
                    let y_index = ((j as i32 + dj as i32) & 255) as usize;
                    let z_index = ((k as i32 + dk as i32) & 255) as usize;
                    let rand_vec3_index =
                        (self.x[x_index] ^ self.y[y_index] ^ self.z[z_index]) as usize;
                    c[di][dj][dk] = self.rand_vec3[rand_vec3_index];
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }

    fn turb(&self, p: &Vec3, depth: i8) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    fn gen_rand_vec3_list() -> [Vec3; 256] {
        let mut p: [Vec3; 256] = [Vec3::new(0.0, 0.0, 0.0); 256];
        let mut rng = thread_rng();
        for i in 0..256 {
            p[i] = Vec3::new(
                -1.0 + 2.0 * rng.gen::<f32>(),
                -1.0 + 2.0 * rng.gen::<f32>(),
                -1.0 + 2.0 * rng.gen::<f32>(),
            )
            .unit_vector();
        }
        p
    }

    fn permute(p: &mut [i32; 256]) {
        let mut rng = thread_rng();
        p.shuffle(&mut rng);
    }

    fn gen_rand_int_list() -> [i32; 256] {
        let mut p: [i32; 256] = [0; 256];
        for i in 0..256 {
            p[i] = i as i32;
        }
        Self::permute(&mut p);
        p
    }
}

lazy_static! {
    static ref PERLIN: Perlin = Perlin::new();
}
