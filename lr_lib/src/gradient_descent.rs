#[inline]
pub fn first_degree(theta_1: f64, error: f64, gamma: f64, max_iter: u64) -> f64 {
    let mut c_error: f64 = 1.0 + error;
    let mut i: u64 = 0;
    let mut dx: f64 = theta_1.clone();

    while error < c_error {
        let p_error: f64 = dx;
        dx = -1.0 * theta_1 * p_error * gamma;
        c_error = (p_error - dx).abs();
        i += 1;
        if i >= max_iter {
            break;
        }
    }
    dx
}
