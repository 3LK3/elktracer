pub fn random_f64_0_1() -> f64 {
    fastrand::f64()
}

pub fn random_f64_m1_1() -> f64 {
    (fastrand::f64() * 2.0) - 1.0
}
