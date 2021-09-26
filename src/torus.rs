use crate::{hit::Hittable, Color, Hit, Ray};

/// A [`Hittable`] torus.
#[derive(Copy, Clone, Debug)]
pub(crate) struct Torus {
    pub(crate) radius_major: f64,
    pub(crate) radius_minor: f64,
}

impl Hittable for Torus {
    fn hit(&self, incidence: Ray) -> Option<Hit> {
        let Ray { origin, direction } = incidence;

        let d_len_sq = direction.len_squared();
        let o_dot_d = origin.dot(direction);
        let p = origin.len_squared() - self.radius_major.powi(2) - self.radius_minor.powi(2);

        let four_major_sq = 4.0 * self.radius_major.powi(2);

        let a = d_len_sq.powi(2);
        let a_3 = 4.0 * o_dot_d / d_len_sq;
        let a_2 = 2.0 * p / d_len_sq
            + (4.0 * o_dot_d.powi(2) + four_major_sq * direction.y().powi(2)) / a;
        let d = 4.0 * o_dot_d * p + 2.0 * four_major_sq * origin.y() * direction.y();
        let e = p.powi(2) - four_major_sq * (self.radius_minor.powi(2) - origin.y().powi(2));

        let solution = IntoIterator::into_iter(solve_quartic(a_3, a_2, d / a, e / a))
            .filter(|n| n.is_finite() && *n >= 0.0)
            .reduce(f64::min);

        solution.map(|t| Hit {
            color: Color::White,
            t,
        })
    }
}

/// Solves a cubic equation of the form x^4 * a_3 * x^3 + a_2 * x^2 + a_1 * x + a_0 = 0.
///
/// Returns potentially NaN solutions in descending order.
///
/// Uses the [NBS method](https://quarticequations.com/Quartic.pdf).
fn solve_quartic(a_3: f64, a_2: f64, a_1: f64, a_0: f64) -> [f64; 4] {
    let u = solve_cubic_greatest(
        -a_2,
        a_1 * a_3 - 4.0 * a_0,
        4.0 * a_0 * a_2 - a_1.powi(2) - a_0 * a_3.powi(2),
    );
    let p_lhs = a_3 / 2.0;
    let p_rhs = (a_3.powi(2) / 4.0 + u - a_2).sqrt();
    let (p_1, p_2) = (p_lhs - p_rhs, p_lhs + p_rhs);
    let q_lhs = u / 2.0;
    let mut q_rhs = (u.powi(2) / 4.0 - a_0).sqrt();
    if a_1 - a_3 * u / 2.0 <= 0.0 {
        q_rhs = -q_rhs;
    }
    let (q_1, q_2) = (q_lhs + q_rhs, q_lhs - q_rhs);
    let (lhs_1, lhs_2) = (-p_1 / 2.0, -p_2 / 2.0);
    let (rhs_1, rhs_2) = (
        (p_1.powi(2) / 4.0 - q_1).sqrt(),
        (p_2.powi(2) / 4.0 - q_2).sqrt(),
    );
    [lhs_1 + rhs_1, lhs_1 - rhs_1, lhs_2 + rhs_2, lhs_2 - rhs_2]
}

/// Solves a cubic equation of the form x^3 + a_2 * x^2 + a_1 * x + a_0 = 0.
/// Returns the greatest solution.
///
/// Uses the [Cardano-Viète Algorithm](https://quarticequations.com/Cubic.pdf).
fn solve_cubic_greatest(a_2: f64, a_1: f64, a_0: f64) -> f64 {
    let q = a_1 / 3.0 - a_2.powi(2) / 9.0;
    let r = (a_1 * a_2 - 3.0 * a_0) / 6.0 - a_2.powi(3) / 27.0;
    let s = r.powi(2) + q.powi(3);
    if s > 0.0 {
        let t = s.sqrt();
        let u = (r + t).cbrt();
        let v = (r - t).cbrt();
        u + v - a_2 / 3.0
    } else {
        let sqrt_neg_q = (-q).sqrt();
        let theta = if q < 0.0 {
            (r / sqrt_neg_q.powi(3)).acos()
        } else {
            0.0
        };
        2.0 * sqrt_neg_q * (theta / 3.0).cos() - a_2 / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_cubic_greatest, solve_quartic};
    use crate::util::check_about;

    #[test]
    fn cubic() {
        check_about(solve_cubic_greatest(2.0, -2.0, 1.0), -2.831);
        check_about(solve_cubic_greatest(2.0, -2.0, -1.0), 1.0);
    }

    #[test]
    fn quartic() {
        let &[x_1, x_2, x_3, x_4] = &solve_quartic(-2.5, 0.8, 1.0, -0.25);
        check_about(x_1, 1.778);
        check_about(x_2, 1.054);
        check_about(x_3, 0.235);
        check_about(x_4, -0.567);
    }
}
