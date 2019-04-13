use super::super::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vec3::Vec3,
};
use std::f32::{consts::PI, MAX as F32MAX};

pub struct FlipNormals<T: Hitable> {
    hitable: T,
}

impl<T: Hitable> Hitable for FlipNormals<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rec = self.hitable.hit(r, t_min, t_max)?;
        Some(HitRecord {
            normal: -rec.normal,
            ..rec
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.hitable.bounding_box()
    }
}

impl<T: Hitable> Translation for FlipNormals<T> {}

pub fn flip_normals<T: Hitable>(hitable: T) -> FlipNormals<T> {
    FlipNormals { hitable: hitable }
}

pub struct Shift<T: Hitable> {
    hitable: T,
    offset: Vec3,
}

impl<T: Hitable> Hitable for Shift<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray {
            origin: r.origin - self.offset,
            direction: r.direction,
        };
        if let Some(rec) = self.hitable.hit(&moved_ray, t_min, t_max) {
            let new_rec = HitRecord {
                p: rec.p + self.offset,
                ..rec
            };
            Some(new_rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        if let Some(original_aabb) = self.hitable.bounding_box() {
            Some(AABB {
                min: original_aabb.min + self.offset,
                max: original_aabb.max + self.offset,
            })
        } else {
            None
        }
    }
}

impl<T: Hitable> Translation for Shift<T> {}

pub struct YRotation<T: Hitable> {
    hitable: T,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl<T: Hitable> YRotation<T> {
    fn gen_bbox(hitable_bbox: AABB, cos_theta: f32, sin_theta: f32) -> AABB {
        let mut min = Vec3::new(F32MAX, F32MAX, F32MAX);
        let mut max = Vec3::new(-F32MAX, -F32MAX, -F32MAX);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * hitable_bbox.max.x + (1 - i) as f32 * hitable_bbox.min.x;
                    let y = j as f32 * hitable_bbox.max.y + (1 - j) as f32 * hitable_bbox.min.y;
                    let z = k as f32 * hitable_bbox.max.z + (1 - k) as f32 * hitable_bbox.min.z;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        let val = tester.index(c);
                        if val > max.index(c) {
                            max.set_index(c, val);
                        }

                        if val < min.index(c) {
                            min.set_index(c, val);
                        }
                    }
                }
            }
        }
        AABB { min, max }
    }

    /// Rotate a hitable about the y axis by angle in degrees
    pub fn new(hitable: T, angle: f32) -> YRotation<T> {
        let radians = (PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let hitable_bbox_maybe = hitable.bounding_box();
        let bbox = if let Some(hitable_bbox) = hitable_bbox_maybe {
            Some(Self::gen_bbox(hitable_bbox, cos_theta, sin_theta))
        } else {
            None
        };

        YRotation {
            hitable,
            bbox,
            sin_theta,
            cos_theta,
        }
    }
}

impl<T: Hitable> Hitable for YRotation<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;
        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;
        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;
        let rotated_ray = Ray { origin, direction };
        let rec = self.hitable.hit(&rotated_ray, t_min, t_max)?;
        let mut p = rec.p;
        let mut normal = rec.normal;
        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
        Some(HitRecord { p, normal, ..rec })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.bbox
    }
}

impl<T: Hitable> Translation for YRotation<T> {}

pub trait Translation: Hitable + Sized {
    fn shift(self, x: f32, y: f32, z: f32) -> Shift<Self> {
        Shift {
            hitable: self,
            offset: Vec3::new(x, y, z),
        }
    }

    fn rotate_y(self, angle: f32) -> YRotation<Self> {
        YRotation::new(self, angle)
    }

    fn flip_normals(self) -> FlipNormals<Self> {
        flip_normals(self)
    }

    fn to_box(self) -> Box<Self> {
        Box::new(self)
    }

    /// Push self into a list of boxed hitables (boxes self)
    fn push_into_list_of_boxed_hitables<'a>(self, list: &mut Vec<Box<Hitable + 'a>>)
    where
        Self: 'a,
    {
        list.push(self.to_box())
    }
}
