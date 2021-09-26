#[cfg(test)]
pub(crate) fn check_about(lhs: f64, rhs: f64) {
    assert!(lhs.is_finite());
    assert!(rhs.is_finite());
    assert!(
        (rhs - lhs).abs() < 1e-3,
        "expected {} to about equal {}",
        lhs,
        rhs
    );
}
