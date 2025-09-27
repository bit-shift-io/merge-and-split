pub fn float_approx_equal(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}