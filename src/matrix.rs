use std::ops;

use crate::{Transformation, Vector};

/// A 3x3 matrix.
#[derive(Copy, Clone, Debug)]
pub(crate) struct Matrix(
    /// The first row of this matrix.
    Vector,
    /// The second row of this matrix.
    Vector,
    /// The third row of this matrix.
    Vector,
);

impl Matrix {
    /// Returns the identity matrix.
    pub(crate) fn identity() -> Matrix {
        Self(
            Vector(1.0, 0.0, 0.0),
            Vector(0.0, 1.0, 0.0),
            Vector(0.0, 0.0, 1.0),
        )
    }

    /// Returns a matrix that causes a rotation of `angle` in the x-axis.
    pub(crate) fn rotation_x(angle: f64) -> Matrix {
        let (sin, cos) = angle.sin_cos();
        Self(
            Vector(1.0, 0.0, 0.0),
            Vector(0.0, cos, -sin),
            Vector(0.0, sin, cos),
        )
    }

    /// Returns a matrix that causes a rotation of `angle` in the y-axis.
    pub(crate) fn rotation_y(angle: f64) -> Matrix {
        let (sin, cos) = angle.sin_cos();
        Self(
            Vector(cos, 0.0, sin),
            Vector(0.0, 1.0, 0.0),
            Vector(-sin, 0.0, cos),
        )
    }

    /// Returns a matrix that causes a rotation of `angle` in the z-axis.
    pub(crate) fn rotation_z(angle: f64) -> Matrix {
        let (sin, cos) = angle.sin_cos();
        Self(
            Vector(cos, -sin, 0.0),
            Vector(sin, cos, 0.0),
            Vector(0.0, 0.0, 1.0),
        )
    }

    /// Returns a matrix that causes a scaling of `factor` in the corresponding axes.
    pub(crate) fn scale(Vector(x, y, z): Vector) -> Matrix {
        Self(
            Vector(x, 0.0, 0.0),
            Vector(0.0, y, 0.0),
            Vector(0.0, 0.0, z),
        )
    }

    /// Returns the determinant of this matrix.
    pub(crate) fn determinant(self) -> f64 {
        self.0 .0 * (self.1 .1 * self.2 .2 - self.2 .1 * self.1 .2)
            - self.0 .1 * (self.1 .0 * self.2 .2 - self.2 .0 * self.1 .2)
            + self.0 .2 * (self.1 .0 * self.2 .1 - self.2 .0 * self.1 .1)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0 .0 + self.1 * rhs.0 .1 + self.2 * rhs.0 .2,
            self.0 * rhs.1 .0 + self.1 * rhs.1 .1 + self.2 * rhs.1 .2,
            self.0 * rhs.2 .0 + self.1 * rhs.2 .1 + self.2 * rhs.2 .2,
        )
    }
}

impl ops::MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Mul<f64> for Matrix {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Matrix> for Vector {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Self(self.dot(rhs.0), self.dot(rhs.1), self.dot(rhs.2))
    }
}

impl ops::MulAssign<Matrix> for Vector {
    fn mul_assign(&mut self, rhs: Matrix) {
        *self = *self * rhs;
    }
}

impl ops::Div<f64> for Matrix {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Matrix {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Transformation for Matrix {
    fn transform(&self, vector: Vector) -> Vector {
        vector * *self
    }

    fn inverse(&self) -> Self {
        Matrix(
            Vector(
                self.1 .1 * self.2 .2 - self.2 .1 * self.1 .2,
                self.0 .2 * self.2 .1 - self.2 .2 * self.0 .1,
                self.0 .1 * self.1 .2 - self.1 .1 * self.0 .2,
            ),
            Vector(
                self.1 .2 * self.2 .0 - self.2 .2 * self.1 .0,
                self.0 .0 * self.2 .2 - self.2 .0 * self.0 .2,
                self.0 .2 * self.1 .0 - self.1 .2 * self.0 .0,
            ),
            Vector(
                self.1 .0 * self.2 .1 - self.2 .0 * self.1 .1,
                self.0 .1 * self.2 .0 - self.2 .1 * self.0 .0,
                self.0 .0 * self.1 .1 - self.1 .0 * self.0 .1,
            ),
        ) / self.determinant()
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::{util::check_about, Transformation, Vector};

    #[test]
    fn matrix_mul_matrix() {
        let a = Matrix(
            Vector(4.0, 8.0, -0.2),
            Vector(-5.5, 0.0, 3.4),
            Vector(6.7, 10.8, 9.9),
        );
        let b = Matrix(
            Vector(7.7, 1.0, 3.3),
            Vector(8.0, 6.3, -4.4),
            Vector(2.5, -0.1, 2.7),
        );
        let c = a * b;
        let v = Vector(5.0, 2.0, -1.5);

        let left = v * a * b;
        let right = v * c;

        check_about(left.0, right.0);
        check_about(left.1, right.1);
        check_about(left.2, right.2);
    }

    #[test]
    fn inverse() {
        let a = Matrix(
            Vector(4.0, 8.0, -0.2),
            Vector(-5.5, 0.0, 3.4),
            Vector(6.7, 10.8, 9.9),
        );
        let identity = a * a.inverse();
        check_about(identity.0 .0, 1.0);
        check_about(identity.0 .1, 0.0);
        check_about(identity.0 .2, 0.0);
        check_about(identity.1 .0, 0.0);
        check_about(identity.1 .1, 1.0);
        check_about(identity.1 .2, 0.0);
        check_about(identity.2 .0, 0.0);
        check_about(identity.2 .1, 0.0);
        check_about(identity.2 .2, 1.0);
    }
}
