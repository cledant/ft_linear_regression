use csv::Reader;
use price;
use types::{Factors, ParsingData};

use std::error::Error;

pub fn compute_factors(
    filename: &String,
    initial_factors: &Factors,
    learning_rate: &f64,
) -> Result<Factors, Box<Error>> {
    //Csv reader
    let mut rdr = Reader::from_path(filename)?;
    let mut rdr_iter = rdr.deserialize();
    //Csv data vector
    let mut list: Vec<ParsingData> = Vec::new();
    while let Some(result) = rdr_iter.next() {
        list.push(result?);
    }
	println!("Ok");
    let mut iter_list = list.iter();
    //Other values
    let coeff = (1.0 / list.len() as f64) * learning_rate;
    let mut new_factors = initial_factors.clone();

    while let Some(val) = iter_list.next() {
        let error = price::estimate_price(&new_factors, &val.mileage) - val.price;
        new_factors.theta_0 += coeff * error;
        new_factors.theta_1 += coeff * error * val.mileage;
    }
    Ok(new_factors)
}
