use std::f64::consts::PI;

pub(crate) fn ease_sin_in_out(x: f64) -> f64 {
    -((PI * x).cos() - 1.0) / 2.0
}
