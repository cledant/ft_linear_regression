extern crate dotenv;
extern crate lr_lib;

use lr_lib::price;
use lr_lib::types::Factors;

use std::env;
use std::error::Error;
use std::io;

#[inline]
fn run(factors: Factors) -> Result<(), Box<Error>> {
    let mut buff = String::new();

    loop {
        buff.clear();
        println!("Enter a mileage : ");
        if let 0 = io::stdin().read_line(&mut buff)? {
            break;
        } else {
            match buff.trim().parse::<f64>() {
                Ok(mileage) => println!(
                    "Estimated price :\n{}",
                    price::estimate_price(&factors, &mileage)
                ),
                Err(e) => println!("Error : {}", e),
            }
        }
    }
    Ok(())
}

fn main() {
    dotenv::dotenv().ok();

    let env_factors = Factors {
        theta_0: env::var("THETA_0")
            .unwrap_or_else(|_| "0.0".to_string())
            .parse::<f64>()
            .unwrap_or_else(|_| 0.0),
        theta_1: env::var("THETA_1")
            .unwrap_or_else(|_| "0.0".to_string())
            .parse::<f64>()
            .unwrap_or_else(|_| 0.0),
    };
    if let Err(err) = run(env_factors) {
        println!("predict : {}", err);
    }
}
