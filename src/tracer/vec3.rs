use rand::prelude::*;
use std::{fmt, ops};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

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

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = thread_rng();
        let mut rnd = || rng.gen::<f32>();
        let mut sample = || 2.0 * Vec3::new(rnd(), rnd(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        let mut p = sample();
        while p.dot(&p) >= 1.0 {
            p = sample()
        }
        p
    }

    pub fn dot(&self, v2: &Vec3) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v2.z - self.z * v2.y,
            y: -(self.x * v2.z - self.z * v2.x),
            z: self.x * v2.y - self.y * v2.x,
        }
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Return a unit vector version of this Vec3
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    /// Field access by index 0: x, 1: y, 2: z
    pub fn index(&self, i: i32) -> f32 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid Vec3 index"),
        }
    }

    /// Field access by index 0: x, 1: y, 2: z
    pub fn set_index(&mut self, i: i32, val: f32) {
        match i {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.z = val,
            _ => panic!("Invalid Vec3 index"),
        }
    }

    pub fn distance_from(self, vec: Vec3) -> f32 {
        ((self.x - vec.x).powi(2) + (self.y - vec.y).powi(2) + (self.z - vec.z).powi(2)).sqrt()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.x.to_string())?;
        fmt.write_str(" ")?;
        fmt.write_str(&self.y.to_string())?;
        fmt.write_str(" ")?;
        fmt.write_str(&self.z.to_string())?;
        Ok(())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, _rhs: Vec3) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, _rhs: Vec3) {
        self.x *= _rhs.x;
        self.y *= _rhs.y;
        self.z *= _rhs.z;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, _rhs: Vec3) {
        self.x /= _rhs.x;
        self.y /= _rhs.y;
        self.z /= _rhs.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, _rhs: f32) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, _rhs: f32) {
        let k = 1.0 / _rhs;
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::FloatCmp;

    #[test]
    fn mul_vec3_vec3() {
        let v1 = Vec3::new(2.0, 1.0, 3.0);
        let v2 = Vec3::new(2.0, 5.0, 4.0);
        let result = v1 * v2;
        result.x.assert_nearly_eq(4.0);
        result.y.assert_nearly_eq(5.0);
        result.z.assert_nearly_eq(12.0);
    }

    #[test]
    fn vec3_to_unit_vector() {
        let unit_vec = Vec3::new(3.4, 3.2, 5.3).unit_vector();
        unit_vec.x.assert_nearly_eq(0.4813624);
        unit_vec.y.assert_nearly_eq(0.45304698);
        unit_vec.z.assert_nearly_eq(0.75035906);
    }
}
