use std::f64::consts::FRAC_PI_2;

use crate::{Color, Hit, Hittable, HittableExt, Matrix, Ray, Translation, Vector};

/// Returns a cube, centered at the origin, with a side length of 1.
pub(crate) fn cube() -> impl Hittable + Copy {
    // A side of the cube. This is a square in the xz plane, centered at the origin and
    // with a side length of 1.
    #[derive(Copy, Clone)]
    struct Side;

    impl Hittable for Side {
        fn hit(&self, incidence: Ray) -> Option<Hit> {
            // First, determine the point at y = 0.
            let t = -incidence.origin.y() / incidence.direction.y();
            let point = incidence.at(t);
            // Check if the point is inside the boundaries
            if -0.5 <= point.x() && point.x() <= 0.5 && -0.5 <= point.z() && point.z() <= 0.5 {
                Some(Hit {
                    color: Color::Red,
                    t,
                })
            } else {
                None
            }
        }
    }

    // Translate to get the two xz sides.
    let y1 = Side.transform(Translation(Vector(0.0, -0.5, 0.0)));
    let y2 = Side
        .transform(Translation(Vector(0.0, 0.5, 0.0)))
        .colorize(Color::Yellow);

    // Rotate in the x-axis for xy sides.
    let z1 = y1
        .transform(Matrix::rotation_x(FRAC_PI_2))
        .colorize(Color::Blue);
    let z2 = y1
        .transform(Matrix::rotation_x(-FRAC_PI_2))
        .colorize(Color::Green);

    // Rotate in the z-axis for yz sides.
    let x1 = y1
        .transform(Matrix::rotation_z(FRAC_PI_2))
        .colorize(Color::Cyan);
    let x2 = y1
        .transform(Matrix::rotation_z(-FRAC_PI_2))
        .colorize(Color::Magenta);

    // Combine all of the sides together.
    y1.and(y2).and(z1).and(z2).and(x1).and(x2)
}
