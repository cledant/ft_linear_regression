use csv::ReaderBuilder;

use types::{Data, Parsing};

use std::error::Error;

#[inline]
pub fn parse_csv_file(filename: &String) -> Result<Parsing, Box<Error>> {
    let mut parsed_data = Parsing {
        data: Vec::new(),
        max_price: 0.0,
        max_mileage: 0.0,
    };
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut rdr_iter = rdr.deserialize();

    while let Some(value) = rdr_iter.next() {
        if let Ok(val) = value {
            let data: Data = val;
            parsed_data.max_mileage = parsed_data.max_mileage.max(data.mileage);
            parsed_data.max_price = parsed_data.max_price.max(data.price);
            parsed_data.data.push(data);
        }
    }
    Ok(parsed_data)
}

#[inline]
pub fn scale_data(parsed_data: &mut Parsing) {
    for val in parsed_data.data.iter_mut() {
        val.mileage /= parsed_data.max_mileage;
        val.price /= parsed_data.max_price;
    }
}
