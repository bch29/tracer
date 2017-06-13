use types::*;
use material::*;

use cgmath::{Matrix4};

pub trait Object {
    fn trace_ray(&self, ray: Ray) -> Option<Intersection>;
    fn material_at(&self, pt: P3) -> &Material;
}

pub struct Transformed<O> {
    object: O,
    mat: Matrix4<f64>,
    inv_mat: Matrix4<f64>,
}

impl<O: Object> Object for Transformed<O> {
    fn trace_ray(&self, ray: Ray) -> Option<Intersection> {
        let local_origin = self.inv_mat.transform_point(ray.origin);
        let local_dir = self.inv_mat.transform_vector(ray.direction);

        let local_ray = Ray {
            origin: local_origin,
            direction: local_dir
        };

        self.object
            .trace_ray(local_ray)
            .map(|intersection| {
                Intersection {
                    position: self.mat.transform_point(intersection.position),
                    normal: self.inv_mat
                        .transpose()
                        .transform_vector(intersection.normal),
                }
            })
    }

    fn material_at(&self, pt: P3) -> &Material {
        self.object.material_at(self.inv_mat.transform_point(pt))
    }
}

