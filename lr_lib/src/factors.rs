use csv::ReaderBuilder;

use price;
use gradient_descent;
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
			let coeff = gradient_descent::first_degree(new_factors.theta_1.clone(),
							error.clone(),
							0.01,
							100); 
			new_factors.theta_0 += coeff * error;
        	new_factors.theta_1 += coeff * error * parsed_data.mileage;
			i += 1;
		}
    }
    Ok(new_factors)
}
