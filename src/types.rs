
pub use cgmath::*;

pub type V3 = Vector3<f64>;
pub type P3 = Point3<f64>;

pub fn v3(x: f64, y: f64, z: f64) -> V3 {
    Vector3::new(x, y, z)
}

pub fn p3(x: f64, y: f64, z: f64) -> P3 {
    Point3::new(x, y, z)
}

#[derive(Clone)]
pub struct Ray {
    pub origin: P3,
    pub direction: V3
}

#[derive(Clone)]
pub enum RayResult {
    Miss,
    Hit {
        reflected_intensity: V3
    }
}

#[derive(Clone)]
pub struct Intersection {
    pub position: P3,
    pub normal: V3
}

pub struct Light {
    pub position: P3,
    pub intensity: V3
}

pub struct SceneParams {
    pub lights: Vec<Light>,
    pub ambient_intensity: V3,
}
