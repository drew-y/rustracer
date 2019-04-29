use super::super::{
    bounding_box::BoundingBox,
    hitable::{HitRecord, Hitable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};
use super::{
    bvh::BVHNode,
    rect::{XYRect, XZRect, YZRect},
    translation::Translation,
};

pub struct Cuboid {
    rects: BVHNode,
}

impl Cuboid {
    pub fn new(pmin: Vec3, pmax: Vec3, material: Material) -> Cuboid {
        let mut list: Vec<Box<Hitable>> = Vec::with_capacity(6);

        XYRect {
            x0: pmin.x,
            x1: pmax.x,
            y0: pmin.y,
            y1: pmax.y,
            k: pmax.z,
            material: material.clone(),
        }
        .push_into_list_of_boxed_hitables(&mut list);

        XYRect {
            x0: pmin.x,
            x1: pmax.x,
            y0: pmin.y,
            y1: pmax.y,
            k: pmin.z,
            material: material.clone(),
        }
        .flip_normals()
        .push_into_list_of_boxed_hitables(&mut list);

        XZRect {
            x0: pmin.x,
            x1: pmax.x,
            z0: pmin.z,
            z1: pmax.z,
            k: pmax.y,
            material: material.clone(),
        }
        .push_into_list_of_boxed_hitables(&mut list);

        XZRect {
            x0: pmin.x,
            x1: pmax.x,
            z0: pmin.z,
            z1: pmax.z,
            k: pmin.y,
            material: material.clone(),
        }
        .flip_normals()
        .push_into_list_of_boxed_hitables(&mut list);

        YZRect {
            y0: pmin.y,
            y1: pmax.y,
            z0: pmin.z,
            z1: pmax.z,
            k: pmax.x,
            material: material.clone(),
        }
        .push_into_list_of_boxed_hitables(&mut list);

        YZRect {
            y0: pmin.y,
            y1: pmax.y,
            z0: pmin.z,
            z1: pmax.z,
            k: pmin.x,
            material: material.clone(),
        }
        .flip_normals()
        .push_into_list_of_boxed_hitables(&mut list);

        Cuboid {
            rects: BVHNode::new(list),
        }
    }
}

impl Hitable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.rects.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        self.rects.bounding_box()
    }
}

impl Translation for Cuboid {}
