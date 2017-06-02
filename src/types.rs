

pub type V3 = Vector3<f64>;
pub type P3 = Point3<f64>;

pub fn v3(x: f64, y: f64, z: f64) -> V3 {
    Vector3::new(x, y, z)
}

pub fn p3(x: f64, y: f64, z: f64) -> P3 {
    Point3::new(x, y, z)
}
