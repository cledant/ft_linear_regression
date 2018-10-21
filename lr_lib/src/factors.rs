use csv::ReaderBuilder;
use price;
use types::{Factors, ParsingData};

use std::error::Error;

pub fn compute_factors(
    filename: &String,
    initial_factors: &Factors,
    learning_rate: &f64,
) -> Result<Factors, Box<Error>> {
    //Csv reader
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut rdr_iter = rdr.deserialize();
    //Csv data vector
    let mut list: Vec<ParsingData> = Vec::new();
    while let Some(result) = rdr_iter.next() {
        if let Ok(res) = result {
            list.push(res);
        }
    }
    let mut iter_list = list.iter();
    //Other values
    let coeff = (1.0 / list.len() as f64) * learning_rate;
    let mut new_factors = initial_factors.clone();

    //println!("coeff : {}", coeff);
    while let Some(val) = iter_list.next() {
        //println!("new_factors : {}, {}", new_factors.theta_0, new_factors.theta_1);
        let error = price::estimate_price(initial_factors, &val.mileage) - val.price;
        //println!("error : {}", error);
        new_factors.theta_0 += coeff * error;
        new_factors.theta_1 += coeff * error * val.mileage;
    }
    Ok(new_factors)
}
