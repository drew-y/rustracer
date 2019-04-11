use super::super::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
};

pub struct FlipNormals<T: Hitable> {
    hitable: T,
}

impl<T: Hitable> Hitable for FlipNormals<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(rec) = self.hitable.hit(r, t_min, t_max) {
            Some(HitRecord {
                normal: -rec.normal,
                ..rec
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.hitable.bounding_box()
    }
}

impl<T: Hitable> Translation for FlipNormals<T> {}

pub fn flip_normals<T: Hitable>(hitable: T) -> FlipNormals<T> {
    FlipNormals { hitable: hitable }
}

pub trait Translation: Hitable + Sized {
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
