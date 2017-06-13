use types::*;


pub trait Material {
    /// The given ray intersected an object with this material. Return a list of
    /// rays to cast out that are needed to calculate the color.
    fn next_rays(&self, scene: &SceneParams, ray: Ray, intersection: Intersection) -> Vec<Ray>;

    /// Once the next rays have been cast and returned their results, use the
    /// results to return the intensity of the light reflected by the material.
    fn collect_rays(&self, scene: &SceneParams, ray: Ray, intersection: Intersection, results: Vec<(Ray, RayResult)>) -> V3;
}
