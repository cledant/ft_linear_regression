use csv::Reader;
use price;
use types;
use types::{Factors, ParsingData};

use std::error::Error;

pub fn compute_factors(
    filename: &String,
    initial_factors: &Factors,
) -> Result<Factors, Box<Error>> {
    let mut rdr = Reader::from_path(filename)?;
    let mut iter = rdr.deserialize();
    let mut new_factors = initial_factors.clone();

    while let Some(result) = iter.next() {
        let record: ParsingData = result?;
        let mut tmp = Factors {
            theta_0: 0.0,
            theta_1: 0.0,
        };
        let error = price::estimate_price(&new_factors, &record.mileage) - record.price;

        tmp.theta_0 = new_factors.theta_0 - types::LEARNING_RATE * error;
        tmp.theta_1 = new_factors.theta_1 - types::LEARNING_RATE * error * record.mileage;
        new_factors = tmp;
    }
    Ok(new_factors)
}
