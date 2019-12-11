use super::super::tracer::*;
use std::f32::{consts::PI, MAX as F32MAX};

#[derive(Clone)]
pub struct FlipNormals {
    hitable: BoxHitable,
}

impl Hitable for FlipNormals {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rec = self.hitable.hit(r, t_min, t_max)?;
        Some(HitRecord {
            normal: -rec.normal,
            ..rec
        })
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        self.hitable.bounding_box()
    }

    fn box_clone(&self) -> BoxHitable {
        Box::new(self.clone())
    }
}

impl Translation for FlipNormals {}

pub fn flip_normals(hitable: BoxHitable) -> FlipNormals {
    FlipNormals { hitable }
}

#[derive(Clone)]
pub struct Shift {
    hitable: BoxHitable,
    offset: Vec3,
}

impl Hitable for Shift {
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

    fn bounding_box(&self) -> Option<BoundingBox> {
        if let Some(original_bounding_box) = self.hitable.bounding_box() {
            Some(BoundingBox {
                min: original_bounding_box.min + self.offset,
                max: original_bounding_box.max + self.offset,
            })
        } else {
            None
        }
    }

    fn box_clone(&self) -> BoxHitable {
        Box::new(self.clone())
    }
}

impl Translation for Shift {}

#[derive(Clone)]
pub struct YRotation {
    hitable: BoxHitable,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<BoundingBox>,
}

impl YRotation {
    fn gen_bbox(hitable_bbox: BoundingBox, cos_theta: f32, sin_theta: f32) -> BoundingBox {
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
        BoundingBox { min, max }
    }

    /// Rotate a hitable about the y axis by angle in degrees
    pub fn new(hitable: BoxHitable, angle: f32) -> YRotation {
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

impl Hitable for YRotation {
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

    fn bounding_box(&self) -> Option<BoundingBox> {
        self.bbox
    }

    fn box_clone(&self) -> BoxHitable {
        Box::new(self.clone())
    }
}

impl Translation for YRotation {}

pub trait Translation: Hitable + Sized {
    fn shift(self, x: f32, y: f32, z: f32) -> Shift {
        Shift {
            hitable: self.box_clone(),
            offset: Vec3::new(x, y, z),
        }
    }

    fn rotate_y(self, angle: f32) -> YRotation {
        YRotation::new(self.box_clone(), angle)
    }

    fn flip_normals(self) -> FlipNormals {
        flip_normals(self.box_clone())
    }

    fn to_box(self) -> Box<Self> {
        Box::new(self)
    }

    /// Push self into a list of boxed hitables (boxes self)
    fn push_into_list_of_boxed_hitables<'a>(self, list: &mut Vec<Box<dyn Hitable + 'a>>)
    where
        Self: 'a,
    {
        list.push(self.to_box())
    }
}
