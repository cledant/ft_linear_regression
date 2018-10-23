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
	//Reading csv
	let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut rdr_iter = rdr.deserialize();
	//Scaling
	let mut max_mileage : f64 = 0.0;
	let mut max_price : f64 = 0.0;
	//Container for data
	let mut vec_value : Vec<ParsingData> = Vec::new();
    while let Some(value) = rdr_iter.next() {
		if let Ok(val) = value {
			let parsed_data : ParsingData = val;
			max_mileage = max_mileage.max(parsed_data.mileage);
			max_price = max_price.max(parsed_data.price);
			vec_value.push(parsed_data);
		}
    }
	//Others
	let coeff = learning_rate / (vec_value.len() as f64);
    let mut new_factors = initial_factors.clone();
	//scaling values
	for val in vec_value.iter_mut() {
		val.mileage /= max_mileage;
		val.price /= max_price;
	}

	for _i in 0..*max_iter {
		let mut tmp_factors = Factors{theta_0 : 0.0, theta_1 : 0.0};
		for parsed_data in vec_value.iter() {
			let error = price::estimate_price(&new_factors, &parsed_data.mileage) - parsed_data.price;
			tmp_factors.theta_0 += coeff * error;
        	tmp_factors.theta_1 += coeff * error * parsed_data.mileage;
		}
		new_factors.theta_0 -= tmp_factors.theta_0;
		new_factors.theta_1 -= tmp_factors.theta_1;
	}
	new_factors.theta_0 *= max_price;
	new_factors.theta_1 *= max_price / max_mileage;
    Ok(new_factors)
}
