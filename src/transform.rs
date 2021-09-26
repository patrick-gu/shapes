use crate::{Ray, Vector};

pub(crate) trait Transformation {
    fn transform(&self, vector: Vector) -> Vector;

    fn inverse(&self) -> Self
    where
        Self: Sized;
}

pub(crate) trait TransformationExt: Transformation {
    fn transform_ray(&self, ray: Ray) -> Ray {
        let origin = self.transform(ray.origin);
        let direction = self.transform(ray.origin + ray.direction) - origin;
        Ray { origin, direction }
    }
}

impl<T: ?Sized + Transformation> TransformationExt for T {}
