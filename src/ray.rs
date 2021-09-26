use crate::Vector;

/// A ray in 3D space, composed of an [`origin`] and a [`direction`].
#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Vector,
    pub(crate) direction: Vector,
}

impl Ray {
    /// Returns a point that is `t` times along this ray.
    pub(crate) fn at(self, t: f64) -> Vector {
        self.origin + t * self.direction
    }
}
