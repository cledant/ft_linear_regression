use price;
use types::{Factors, Parsing};

#[inline]
pub fn compute_factors(
    parsed_data: &Parsing,
    initial_factors: &Factors,
    learning_rate: &f64,
    max_iter: &usize,
) -> Factors {
    let mut new_factors = initial_factors.clone();
    let coeff = learning_rate / (parsed_data.data.len() as f64);

    for _i in 0..*max_iter {
        let mut tmp_factors = Factors {
            theta_0: 0.0,
            theta_1: 0.0,
        };
        for value in parsed_data.data.iter() {
            let error = price::estimate_price(&new_factors, &value.mileage) - value.price;
            tmp_factors.theta_0 += coeff * error;
            tmp_factors.theta_1 += coeff * error * value.mileage;
        }
        new_factors.theta_0 -= tmp_factors.theta_0;
        new_factors.theta_1 -= tmp_factors.theta_1;
    }
    new_factors.theta_0 *= parsed_data.max_price;
    new_factors.theta_1 *= parsed_data.max_price / parsed_data.max_mileage;
    new_factors
}
