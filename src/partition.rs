use types::*;

/// A space-partitioning structure which allows for fast searching of objects in
/// 3D space.
pub struct Partition<O> {
    objects: Vec<O>
}

impl<O> Partition<O> {
    pub fn new() -> Self {
        Partition{
            objects: Vec::new()
        }
    }

    /// Insert a new object into the partitioning structure.
    pub fn insert(&mut self, obj: O) {
        self.objects.push(obj);
    }

    /// Split the partitioning structure into two parts, nearest the given ray
    /// and furthest from the given ray.
    pub fn split(&self, ray: Ray) -> Option<(&Self, &Self)> {
        None
    }

    /// Return all the objects in the partitioned space.
    pub fn all_objects(&self) -> &[O] {
        &*self.objects
    }
}
