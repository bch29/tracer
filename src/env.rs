use tracer::*;

use cgmath::{Matrix4};

pub struct Incidence {
    pub normal: V3,
    pub light_dir: V3,
    pub eye_dir: V3,
    pub light_color: V3
}

pub trait IsoMaterial {
    fn refl_color(incidence: Incidence) -> V3;
}

pub struct Transformed<O> {
    object: O,
    mat: Matrix4<f64>,
    inv_mat: Matrix4<f64>,
}

impl<O: Object> Object for Transformed<O> {
    fn test_hit(&self, origin: P3, ray: V3) -> Option<Intersection> {
        let local_origin = self.inv_mat.transform_point(origin);
        let local_ray = self.inv_mat.transform_vector(ray);

        self.object
            .test_hit(local_origin, local_ray)
            .map(|intersection| {
                Intersection {
                    pos: self.mat.transform_point(intersection.pos),
                    normal: self.inv_mat
                        .transpose()
                        .transform_vector(intersection.normal),
                    specular_k: intersection.specular_k,
                    specular_n: intersection.specular_n,
                    diffuse_k: intersection.diffuse_k,
                    ambient_k: intersection.ambient_k,
                    reflectivity: intersection.reflectivity,
                }
            })
    }
}

// pub struct UnitSphere {
//     specular_k: V3,
//     specular_n: f64,
//     diffuse_k: V3,
//     ambient_k: V3,
//     reflectivity: V3,
// }

// impl Object for UnitSphere {
//     fn test_hit(&self, origin: P3, ray: V3) -> Option<Intersection> {
        
//     }
// }

pub struct Sphere {
    centre: P3,
    radius: f64,
    specular_k: V3,
    specular_n: f64,
    diffuse_k: V3,
    ambient_k: V3,
    reflectivity: V3,
}

impl Object for Sphere {
    fn test_hit(&self, origin: P3, ray: V3) -> Option<Intersection> {
        let a = ray.magnitude2();
        let b = 2.0 * ray.dot(origin - self.centre);
        let c = origin.sub(&self.centre).dot(origin.sub(&self.centre)) - self.radius * self.radius;

        let d2 = b * b - 4.0 * a * c;

        if d2 >= 0.0 {
            let d = d2.sqrt();
            let t = (-b - d) / (2.0 * a);
            if t >= 0.0 {
                let p = origin + t * ray;

                let n = p.sub(&self.centre).normalize();

                Some(Intersection {
                         pos: p,
                         normal: n,
                         specular_k: self.specular_k,
                         specular_n: self.specular_n,
                         diffuse_k: self.diffuse_k,
                         ambient_k: self.ambient_k,
                         reflectivity: self.reflectivity,
                     })
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct Plane {
    normal: V3,
    d: f64,
    specular_k: V3,
    specular_n: f64,
    diffuse_k: V3,
    ambient_k: V3,
    reflectivity: V3,
}

impl Object for Plane {
    fn test_hit(&self, origin: P3, ray: V3) -> Option<Intersection> {
        let s = -(self.d + self.normal.dot(origin - Point3::origin())) / self.normal.dot(ray);

        if s >= 0.0 {
            let p = origin.add(&ray.mul(s));
            Some(Intersection {
                     pos: p,
                     normal: self.normal,
                     specular_k: self.specular_k,
                     specular_n: self.specular_n,
                     diffuse_k: self.diffuse_k,
                     ambient_k: self.ambient_k,
                     reflectivity: self.reflectivity,
                 })
        } else {
            None
        }
    }
}

fn shiny_sphere(centre: P3, radius: f64, col: V3) -> Sphere {
    Sphere {
        centre: centre,
        radius: radius,
        specular_k: v3(0.25, 0.25, 0.25).add(&col.mul(0.25)),
        specular_n: 10.0,
        diffuse_k: col.mul(0.4),
        ambient_k: col.mul(0.1),
        reflectivity: v3(0.15, 0.15, 0.15).add(&col.mul(0.2)),
    }
}

fn mk_plane(normal: V3, dist: f64, col: V3) -> Plane {
    Plane {
        normal: normal,
        d: dist,
        specular_k: col.mul(0.2),
        specular_n: 5.0,
        diffuse_k: col.mul(0.4),
        ambient_k: col.mul(0.1),
        reflectivity: v3(0.0, 0.0, 0.0),
    }
}

pub fn default_env() -> Environment {
    let cam_pos = p3(0.0, 0.0, -3.0);
    let cam_dir = v3(0.0, 0.0, 1.0).normalize();
    let cam_img_dist = 2.5;
    let img_w = 1.0;
    let ambient_intensity = v3(0.2, 0.2, 0.2);

    let s1 = Box::new(shiny_sphere(p3(1.8, 0.4, 5.0), 0.5, v3(1.0, 0.0, 0.0)));
    let s2 = Box::new(shiny_sphere(p3(0.0, -0.6, 4.0), 0.5, v3(0.0, 1.0, 1.0)));
    let s3 = Box::new(shiny_sphere(p3(1.8, -0.6, 5.0), 0.5, v3(1.0, 1.0, 0.0)));
    let p1 = Box::new(mk_plane(v3(0.0, 0.0, -1.0).normalize(), 5.5, v3(0.1, 0.1, 0.1)));

    let l1 = Light {
        pos: p3(4.0, -2.0, -3.0),
        intensity: v3(0.5, 0.5, 0.5),
    };

    let l2 = Light {
        pos: p3(-2.0, -1.0, 2.0),
        intensity: v3(0.5, 0.5, 0.5),
    };

    Environment::new(vec![s1, s2, s3, p1],
                     vec![l1, l2],
                     ambient_intensity,
                     cam_pos,
                     cam_dir,
                     cam_img_dist,
                     img_w)
}
