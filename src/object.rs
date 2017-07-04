use types::*;
use bounding_box::*;

use cgmath::{Matrix4};

pub trait Object {
    fn test_ray(&self, ray: &Ray) -> Option<Intersection>;
    fn midpoint(&self) -> P3;
    fn bounding_box(&self) -> BoundingBox;

    fn test_ray_simple(&self, ray: &Ray) -> bool {
        self.test_ray(ray).is_some()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    pub min_corner: P3,
    pub max_corner: P3,
}

impl BoundingBox {
    pub fn expand(&mut self, other: &BoundingBox) {
        self.min_corner.x = self.min_corner.x.min(other.min_corner.x);
        self.min_corner.y = self.min_corner.y.min(other.min_corner.y);
        self.min_corner.z = self.min_corner.z.min(other.min_corner.z);

        self.max_corner.x = self.max_corner.x.max(other.max_corner.x);
        self.max_corner.y = self.max_corner.y.max(other.max_corner.y);
        self.max_corner.z = self.max_corner.z.max(other.max_corner.z);
    }

    // See https://tavianator.com/fast-branchless-raybounding-box-intersections/
    fn test_ray_min_max(&self, ray: &Ray) -> (f64, f64) {
        let tx1 = (self.min_corner.x - ray.origin.x) * ray.direction_inv().x;
        let tx2 = (self.max_corner.x - ray.origin.x) * ray.direction_inv().x;
        let tmin = tx1.min(tx2);
        let tmax = tx1.max(tx2);

        let ty1 = (self.min_corner.y - ray.origin.y) * ray.direction_inv().y;
        let ty2 = (self.max_corner.y - ray.origin.y) * ray.direction_inv().y;
        let tmin = tmin.max(ty1.min(ty2));
        let tmax = tmax.min(ty1.max(ty2));

        let tz1 = (self.min_corner.z - ray.origin.z) * ray.direction_inv().z;
        let tz2 = (self.max_corner.z - ray.origin.z) * ray.direction_inv().z;
        let tmin = tmin.max(tz1.min(tz2));
        let tmax = tmax.min(tz1.max(tz2));

        (tmin, tmax)
    }
}

impl Object for BoundingBox {

    fn test_ray_simple(&self, ray: &Ray) -> bool {
        let (tmin, tmax) = self.test_ray_min_max(ray);
        tmax >= tmin
    }

    fn test_ray(&self, ray: &Ray) -> Option<Intersection> {
        let (tmin, tmax) = self.test_ray_min_max(ray);

        if tmax >= tmin {
            // Some(Intersection {
            //     position: ray.at_t(tmin),
            // })
            unimplemented!()
        } else {
            None
        }
    }

    fn midpoint(&self) -> P3 {
        self.min_corner.midpoint(self.max_corner)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.clone()
    }
}

pub struct Transformed<O> {
    object: O,
    mat: Matrix4<f64>,
    inv_mat: Matrix4<f64>,
}

impl<O: Object> Object for Transformed<O> {
    fn test_ray(&self, ray: &Ray) -> Option<Intersection> {
        let local_ray = Ray::new(
            self.inv_mat.transform_point(ray.origin),
            self.inv_mat.transform_vector(ray.direction),
        );

        self.object
            .test_ray(&local_ray)
            .map(|intersection| {
                Intersection {
                    position: self.mat.transform_point(intersection.position),
                    normal: self.inv_mat
                        .transpose()
                        .transform_vector(intersection.normal),
                }
            })
    }

    fn midpoint(&self) -> P3 {
        self.inv_mat.transform_point(self.object.midpoint())
    }

    fn bounding_box(&self) -> BoundingBox {
        let bb = self.object.bounding_box();

        BoundingBox {
            min_corner: self.inv_mat.transform_point(bb.min_corner),
            max_corner: self.inv_mat.transform_point(bb.max_corner),
        }
    }
}

