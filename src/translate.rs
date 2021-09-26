use crate::{Transformation, Vector};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Translation(pub(crate) Vector);

impl Transformation for Translation {
    fn transform(&self, vector: Vector) -> Vector {
        vector + self.0
    }

    fn inverse(&self) -> Self {
        Self(-self.0)
    }
}
