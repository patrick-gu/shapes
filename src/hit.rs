use crate::{Color, Ray, Transformation, TransformationExt};

/// An object in space that can be hit by a [`Ray`] to possibly produce a [`Hit`].
pub(crate) trait Hittable {
    /// Attempts to hit this object given an `incidence` ray of light,
    fn hit(&self, incidence: Ray) -> Option<Hit>;
}

impl<T: Hittable> Hittable for &T {
    fn hit(&self, incidence: Ray) -> Option<Hit> {
        T::hit(*self, incidence)
    }
}

impl<T: Hittable> Hittable for &mut T {
    fn hit(&self, incidence: Ray) -> Option<Hit> {
        T::hit(*self, incidence)
    }
}

/// Produced when a [`Ray`] hits a [`Hittable`].
#[derive(Copy, Clone, Debug)]
pub(crate) struct Hit {
    pub(crate) color: Color,
    /// The distance [at](Ray::at) the incidence ray where the [`Hittable`] was struck.
    pub(crate) t: f64,
}

/// Extension utilities implemented for all [`Hittable`].
pub(crate) trait HittableExt: Hittable {
    /// Returns a [`Hittable`] that performs a [`Transformation`] before and after
    /// this one.
    fn transform<T: Transformation>(self, transformation: T) -> Transformed<Self, T>
    where
        Self: Sized,
    {
        let transformation_inverse = transformation.inverse();
        Transformed {
            hittable: self,
            transformation_inverse,
        }
    }

    /// Returns a [`Hittable`] that returns [`Hit`]s of a certain [`Color`].
    fn colorize(self, color: Color) -> Colorize<Self>
    where
        Self: Sized,
    {
        Colorize {
            hittable: self,
            color,
        }
    }

    /// Returns a [`Hittable`] that returns the closest [`Hit`] of `self` and `other`.
    fn and<O: Hittable>(self, other: O) -> And<Self, O>
    where
        Self: Sized,
    {
        And {
            first: self,
            second: other,
        }
    }
}

impl<T: ?Sized + Hittable> HittableExt for T {}

/// Created by [`HittableExt::transform`].
#[derive(Copy, Clone, Debug)]
pub(crate) struct Transformed<H, T> {
    hittable: H,
    transformation_inverse: T,
}

impl<H: Hittable, T: Transformation> Hittable for Transformed<H, T> {
    fn hit(&self, incidence: Ray) -> Option<Hit> {
        self.hittable
            .hit(self.transformation_inverse.transform_ray(incidence))
            .map(|Hit { color, t }| Hit { color, t })
    }
}

/// Created by [`HittableExt::colorize`].
#[derive(Copy, Clone, Debug)]
pub(crate) struct Colorize<H> {
    hittable: H,
    color: Color,
}

impl<H: Hittable> Hittable for Colorize<H> {
    fn hit(&self, incidence: Ray) -> Option<Hit> {
        self.hittable.hit(incidence).map(|Hit { t, .. }| Hit {
            color: self.color,
            t,
        })
    }
}

/// Created by [`HittableExt::and`].
#[derive(Copy, Clone, Debug)]
pub(crate) struct And<T, U> {
    first: T,
    second: U,
}

impl<T: Hittable, U: Hittable> Hittable for And<T, U> {
    fn hit(&self, incidence: Ray) -> Option<Hit> {
        match (self.first.hit(incidence), self.second.hit(incidence)) {
            (Some(first), Some(second)) => Some(if first.t <= second.t { first } else { second }),
            (Some(hit), None) | (None, Some(hit)) => Some(hit),
            (None, None) => None,
        }
    }
}
