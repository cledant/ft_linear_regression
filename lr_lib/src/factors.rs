use csv::ReaderBuilder;

use price;
use types::{Factors, ParsingData};

use std::error::Error;

#[inline]
pub fn compute_factors(
    filename: &String,
    initial_factors: &Factors,
    learning_rate: &f64,
	max_iter : &usize,
) -> Result<Factors, Box<Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut rdr_iter = rdr.deserialize();
    let mut new_factors = initial_factors.clone();
	let mut vec_value : Vec<ParsingData> = Vec::new();

    while let Some(value) = rdr_iter.next() {
		if let Ok(val) = value {
			let parsed_data : ParsingData = val;
			vec_value.push(parsed_data);
		}
    }

	let coeff = learning_rate / (vec_value.len() as f64);
	for i in 0..*max_iter {
		let parsed_data = vec_value.get(i % vec_value.len()).unwrap();
		let error = price::estimate_price(&new_factors, &parsed_data.mileage) - parsed_data.price;
		new_factors.theta_0 -= coeff * error;
        new_factors.theta_1 -= coeff * error * parsed_data.mileage;
	}
    Ok(new_factors)
}
