
use types::*;
use object::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn next(self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }

    fn get(&self, point: &P3) -> f64 {
        match *self {
            Axis::X => point.x,
            Axis::Y => point.y,
            Axis::Z => point.z,
        }
    }
}

fn partition_median<T, F>(points: Vec<T>, get_coord: F) -> (Vec<T>, T, Vec<T>)
    where F: FnMut(T) -> f64
{
    unimplemented!()
}

fn partition_median_by_axis<O: Object>(objects: Vec<O>, axis: Axis) -> (Vec<O>, O, Vec<O>) {
    partition_median(objects, |ref o| axis.get(&o.midpoint()))
}

pub struct KdBranch<O> {
    object: O,
    bounding_box: BoundingBox,
    axis: Axis,
    left: KdTree<O>,
    right: KdTree<O>,
}

pub enum KdTree<O> {
    Branch(Box<KdBranch<O>>),
    Leaf,
}

impl<O: Object> KdTree<O> {
    fn from_objects_axis(objects: Vec<O>, axis: Axis) -> KdTree<O> {
        if objects.is_empty() {
            KdTree::Leaf
        } else {
            let (left_objects, median, right_objects) = partition_median_by_axis(objects, axis);

            let next_axis = axis.next();
            let left = KdTree::from_objects_axis(left_objects, next_axis);
            let right = KdTree::from_objects_axis(right_objects, next_axis);

            let mut bounding_box = median.bounding_box();

            match &left {
                &KdTree::Branch(br) => bounding_box.expand(&br.bounding_box),
                _ => (),
            }

            match &right {
                &KdTree::Branch(br) => bounding_box.expand(&br.bounding_box),
                _ => (),
            }

            KdTree::Branch(Box::new(KdBranch {
                                        object: median,
                                        bounding_box,
                                        axis,
                                        left,
                                        right,
                                    }))
        }
    }

    pub fn from_objects(objects: Vec<O>) -> KdTree<O> {
        KdTree::from_objects_axis(objects, Axis::X)
    }

    pub fn insert(&mut self, object: O) {
        match self {
            &mut KdTree::Branch(ref mut br) => unimplemented!(),
            &mut KdTree::Leaf => {
                let new_self = KdBranch {
                    object,
                    bounding_box: object.bounding_box(),
                    axis: Axis::X,
                    left: KdTree::Leaf,
                    right: KdTree::Leaf,
                };

                *self = KdTree::Branch(Box::new(new_self))
            }
        }
    }
}
