use super::super::{
    vec3::Vec3,
    hitable::{ Hitable, HitRecord },
    material::Material,
    aabb::AABB,
    ray::Ray
};
use super::{
    bvh::BVHNode,
    translation,
    rect::{ XYRect, YZRect, XZRect },
};


pub struct BoxGeo {
    rects: BVHNode
}

impl BoxGeo {
    pub fn new(pmin: Vec3, pmax: Vec3, material: Material) -> BoxGeo {
        let mut list: Vec<Box<Hitable>> = Vec::with_capacity(6);

        list.push(Box::new(XYRect {
            x0: pmin.x, x1: pmax.x, y0: pmin.y, y1: pmax.y, k: pmax.z,
            material: material.clone()
        }));

        list.push(Box::new(translation::flip_normals(XYRect {
            x0: pmin.x, x1: pmax.x, y0: pmin.y, y1: pmax.y, k: pmin.z,
            material: material.clone()
        })));

        list.push(Box::new(XZRect {
            x0: pmin.x, x1: pmax.x, z0: pmin.z, z1: pmax.z, k: pmax.y,
            material: material.clone()
        }));

        list.push(Box::new(translation::flip_normals(XZRect {
            x0: pmin.x, x1: pmax.x, z0: pmin.z, z1: pmax.z, k: pmin.y,
            material: material.clone()
        })));

        list.push(Box::new(YZRect {
            y0: pmin.y, y1: pmax.y, z0: pmin.z, z1: pmax.z, k: pmax.x,
            material: material.clone()
        }));

        list.push(Box::new(translation::flip_normals(YZRect {
            y0: pmin.y, y1: pmax.y, z0: pmin.z, z1: pmax.z, k: pmin.x,
            material: material.clone()
        })));

        BoxGeo { rects: BVHNode::new(list) }
    }
}

impl Hitable for BoxGeo {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.rects.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.rects.bounding_box()
    }
}
