use std::ops;

/// A 3D vector of [`f64`]s.
#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Vector(
    /// The x-coordinate of this vector.
    pub(crate) f64,
    /// The y-coordinate of this vector.
    pub(crate) f64,
    /// The z-coordinate of this vector.
    pub(crate) f64,
);

impl Vector {
    /// A vector with zero values.
    pub(crate) const ZERO: Self = Self(0.0, 0.0, 0.0);

    /// Returns the x-coordinate of this vector.
    pub(crate) const fn x(self) -> f64 {
        self.0
    }

    /// Returns the y-coordinate of this vector.
    pub(crate) const fn y(self) -> f64 {
        self.1
    }

    /// Returns the z-coordinate of this vector.
    pub(crate) const fn z(self) -> f64 {
        self.2
    }

    /// Returns the dot product of this vector with another vector
    pub(crate) fn dot(self, rhs: Vector) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    /// Returns the length of this vector, squared.
    pub(crate) fn len_squared(self) -> f64 {
        self.dot(self)
    }

    /// Returns the length of this vector
    #[allow(dead_code)]
    pub(crate) fn len(self) -> f64 {
        self.len_squared().sqrt()
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;
    use crate::util::check_about;

    #[test]
    fn add() {
        let a = Vector(5.0, 6.0, 2.5);
        let b = Vector(1.2, 3.4, 5.6);
        let sum = a + b;
        check_about(sum.x(), 6.2);
        check_about(sum.y(), 9.4);
        check_about(sum.z(), 8.1);
    }

    #[test]
    fn neg() {
        let a = Vector(6.7, 8.0, 9.3);
        let b = -a;
        check_about(b.x(), -6.7);
        check_about(b.y(), -8.0);
        check_about(b.z(), -9.3);
    }
}
