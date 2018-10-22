use csv::ReaderBuilder;
use price;
use types::{Factors, ParsingData};

use std::error::Error;

#[inline]
pub fn compute_factors(
    filename: &String,
    initial_factors: &Factors,
    learning_rate: &f64,
) -> Result<Factors, Box<Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut rdr_iter = rdr.deserialize();
    let mut new_factors = initial_factors.clone();
	let mut i : u64 = 0;

    while let Some(value) = rdr_iter.next() {
		if let Ok(val) = value {
			let parsed_data : ParsingData = val;
	        let error = price::estimate_price(&new_factors, &parsed_data.mileage) - parsed_data.price;
			new_factors.theta_0 -= learning_rate * error;
        	new_factors.theta_1 -= learning_rate * error * parsed_data.mileage;
			i += 1;
			println!("{}", new_factors);
		}
    }
//	let coeff = learning_rate / (i as f64);
//	new_factors.theta_0 *= coeff;
//	new_factors.theta_1 *= coeff;
    Ok(new_factors)
}
