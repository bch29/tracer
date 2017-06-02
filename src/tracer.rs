pub use cgmath::*;
pub use core::ops::Add;
pub use core::ops::Mul;
pub use core::ops::Sub;
pub use core::ops::Neg;

use types::*;

#[derive(Clone)]
pub struct Intersection {
    pub pos: P3,
    pub normal: V3,
    pub specular_k: V3,
    pub specular_n: f64,
    pub diffuse_k: V3,
    pub ambient_k: V3,
    pub reflectivity: V3,
}

pub trait Object: Send {
    /// Test if the ray hits the object, returning the coordinate of the hit if
    /// so.
    fn test_hit(&self, origin: P3, ray: V3) -> Option<Intersection>;
}

pub struct WithId<A> {
    id: usize,
    x: A,
}

pub struct Light {
    pub pos: P3,
    pub intensity: V3,
}

pub struct Environment {
    objects: Vec<WithId<Box<Object + Sync>>>,
    lights: Vec<Light>,
    ambient_intensity: V3,
    cam_pos: P3,
    img_tl: P3,
}

// unsafe impl Sync for Environment {}

#[derive(Clone)]
struct HitResult<'a> {
    object: &'a WithId<Box<Object + Sync>>,
    inter: Intersection,
    dist: f64,
}

trait HasVal {
    fn get_val(&self) -> i32;
}

impl Environment {
    fn trace_ray<'a>(&'a self,
                     origin: P3,
                     dir: V3,
                     ignore: Option<usize>)
                     -> Option<HitResult<'a>> {
        let mut closest: Option<HitResult<'a>> = None;

        for obj in &self.objects {
            if ignore.map_or(true, |id| id != obj.id) {
                obj.x
                    .test_hit(origin, dir)
                    .map(|inter| {
                        let dist = (inter.pos - origin).magnitude();
                        let this_result = HitResult {
                            object: obj,
                            inter: inter,
                            dist: dist,
                        };
                        match closest {
                            None => closest = Some(this_result),
                            Some(r) => {
                                if dist < r.dist {
                                    closest = Some(this_result);
                                }
                            }
                        }
                    });
            }
        }

        closest
    }

    fn get_col_from_pt(&self, origin: P3, ray: V3, ignore: Option<usize>, n: i32) -> V3 {
        let mut illum = v3(0.0, 0.0, 0.0);

        match self.trace_ray(origin, ray, ignore) {
            Some(res) => {
                illum = illum.add(&res.inter.ambient_k.mul_element_wise(self.ambient_intensity));

                let inter = &res.inter;
                let normal = inter.normal;

                for light in &self.lights {
                    let light_dir = (light.pos - inter.pos).normalize();
                    let intensity = light.intensity.mul(1.0 / res.dist * res.dist);

                    let reflect_dir = light_dir
                        .sub(&normal.mul(2.0 * light_dir.dot(normal)))
                        .normalize();

                    let diffuse_dot = light_dir.dot(normal);
                    let spec_dot = reflect_dir.dot(ray);

                    let shadower = self.trace_ray(inter.pos, light_dir, Some(res.object.id));
                    if shadower.is_none() {
                        if spec_dot >= 0.0 {
                            let spec_illum = intensity
                                .mul_element_wise(inter.specular_k)
                                .mul(spec_dot.powf(inter.specular_n));
                            illum = illum.add(&spec_illum);
                        }

                        if diffuse_dot >= 0.0 {
                            let diff_illum =
                                intensity.mul_element_wise(inter.diffuse_k).mul(diffuse_dot);
                            illum = illum.add(&diff_illum);
                        }
                    }
                }

                let ray_reflect_dir = ray.sub(&normal.mul(2.0 * ray.dot(normal))).normalize();

                if n > 0 {
                    let reflect_illum = self.get_col_from_pt(inter.pos,
                                                             ray_reflect_dir,
                                                             Some(res.object.id),
                                                             n - 1);
                    illum = illum.add(&reflect_illum.mul_element_wise(inter.reflectivity));
                }

                illum
            }
            None => v3(0.0, 0.0, 0.0),
        }
    }

    pub fn get_point_col(&self, x: f64, y: f64) -> V3 {
        let plane_pos = p3(x + self.img_tl.x, y + self.img_tl.y, self.img_tl.z);

        let ray = plane_pos.sub(&self.cam_pos).normalize();

        self.get_col_from_pt(plane_pos, ray, None, 3)
    }

    pub fn new(objects: Vec<Box<Object + Sync>>,
               lights: Vec<Light>,
               ambient_intensity: V3,
               cam_pos: P3,
               cam_dir: V3,
               cam_img_dist: f64,
               img_w: f64)
               -> Self {

        let up_vec = cam_dir.cross(v3(1.0, 0.0, 0.0)).normalize();
        let left_vec = up_vec.cross(cam_dir).normalize();

        let img_tl = cam_pos + (cam_dir * cam_img_dist) + (-left_vec * img_w / 2.0) +
                     (-up_vec * img_w / 2.0);

        let mut objects_ids = Vec::with_capacity(objects.len());
        let mut i = 0;

        for o in objects {
            objects_ids.push(WithId { id: i, x: o });
            i += 1;
        }

        Environment {
            objects: objects_ids,
            lights,
            ambient_intensity,
            cam_pos,
            img_tl,
        }
    }
}
