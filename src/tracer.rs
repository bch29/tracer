use types::*;
use partition::*;
use object::*;

pub struct Scene {
    params: SceneParams,
    objects: Partition<Box<Object>>,
    eye_pos: P3,
    image_tl: P3,
}

impl Scene {
    fn trace_partition(&self, part: &Partition<Box<Object>>, ray: Ray) -> RayResult {
        // If the objects can be partitioned, do it and split the trace by partition.
        if let Some((part1, part2)) = part.split(ray.clone()) {
            let res1 = self.trace_partition(part1, ray.clone());
            return match res1.clone() {
                       RayResult::Hit { .. } => res1,
                       RayResult::Miss => self.trace_partition(part2, ray),
                   };
        };

        // Otherwise loop sequentially through objects in the current partition.
        for obj in part.all_objects() {
            if let Some(intersection) = obj.trace_ray(ray.clone()) {
                unimplemented!()
            }
        }

        RayResult::Miss
    }
}
