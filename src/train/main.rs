extern crate dotenv;
extern crate lr_lib;

use lr_lib::factors;
use lr_lib::types::Factors;

use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

static DEFAULT_LEARNING_RATE: f64 = 0.001;
static HELP_MSG: &'static str = "Traning Usage :
	-h : show help
	-f : path to csv file
	-l : set learning rate value. Default is 0.1";

pub struct ParsedArgs {
    filename: Option<String>,
    learning_rate: f64,
    show_help: bool,
}

#[inline]
fn parse_arguments(args: Vec<String>) -> ParsedArgs {
    let mut parsed_args = ParsedArgs {
        filename: None,
        learning_rate: DEFAULT_LEARNING_RATE,
        show_help: false,
    };

    for i in 1..args.len() {
        match args.get(i).unwrap().as_str() {
            "-h" => parsed_args.show_help = true,
            "-f" => {
                parsed_args.filename = match args.get(i + 1) {
                    Some(val) => Some(val.to_string()),
                    None => None,
                };
            }
            "-l" => {
                parsed_args.learning_rate = match args.get(i + 1) {
                    Some(val) => val.parse::<f64>().unwrap_or_else(|_| DEFAULT_LEARNING_RATE),
                    None => DEFAULT_LEARNING_RATE,
                };
            }
            _ => {}
        }
    }
    parsed_args
}

#[inline]
fn save_in_env_file(to_save: &Factors) -> Result<(), Box<Error>> {
    let mut file = OpenOptions::new().write(true).create(true).open(".env")?;

    write!(
        file,
        "THETA_0={}\nTHETA_1={}\n",
        to_save.theta_0.to_string(),
        to_save.theta_1.to_string()
    );
    Ok(())
}

#[inline]
fn run(args: ParsedArgs, factors: Factors) {
    if args.filename == None || args.show_help == true {
        println!("{}", HELP_MSG);
    } else {
        match factors::compute_factors(&args.filename.unwrap(), &factors, &args.learning_rate) {
            Ok(values) => {
                println!("{}", values);
                save_in_env_file(&values)
                    .unwrap_or_else(|err| println!("Saving result error : {}", err))
            }
            Err(err) => println!("Factors computation error : {}", err),
        }
    }
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
    let args: Vec<String> = env::args().collect();
    let parsed_args = parse_arguments(args);
    run(parsed_args, env_factors);
}
