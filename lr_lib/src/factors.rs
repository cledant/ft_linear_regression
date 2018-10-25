use price;
use types::{Factors, Parsing};

#[inline]
pub fn compute_factors(
    parsed_data: &Parsing,
    initial_factors: &Factors,
    learning_rate: &f64,
    max_iter: &usize,
	precision: &f64,
) -> Factors {
    let mut new_factors = initial_factors.clone();
    let coeff = learning_rate / (parsed_data.data.len() as f64);
	let mut i : usize = 0;
	let mut prev_step = 1.0;
	
    while i < *max_iter && prev_step > *precision {
        let mut tmp_factors = Factors {
            theta_0: 0.0,
            theta_1: 0.0,
			ms_error: 0.0,
			stop_iter: 0,
        };
		let prev_error = new_factors.theta_0;
        for value in parsed_data.data.iter() {
            let error = price::estimate_price(&new_factors, &value.mileage) - value.price;
            tmp_factors.theta_0 += error;
            tmp_factors.theta_1 += error * value.mileage;
			tmp_factors.ms_error += (error * error) / (parsed_data.data.len() as f64);
        }
        new_factors.theta_0 -= coeff * tmp_factors.theta_0;
        new_factors.theta_1 -= coeff * tmp_factors.theta_1;
		new_factors.ms_error = tmp_factors.ms_error;
		prev_step = (prev_error - new_factors.theta_0).abs();
		i += 1;
    }
	new_factors.stop_iter = i;
    new_factors.theta_0 *= parsed_data.max_price;
    new_factors.theta_1 *= parsed_data.max_price / parsed_data.max_mileage;
    new_factors
}
