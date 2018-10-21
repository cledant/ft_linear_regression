use types::Factors;

#[inline]
pub fn estimate_price(factors: &Factors, mileage: &f64) -> f64 {
    factors.theta_0 + factors.theta_1 * mileage
}
