use std::fmt;

#[derive(Deserialize)]
pub struct Data {
    pub mileage: f64,
    pub price: f64,
}

pub struct Parsing {
    pub data: Vec<Data>,
    pub max_price: f64,
    pub max_mileage: f64,
}

#[derive(Debug, Clone)]
pub struct Factors {
    pub theta_0: f64,
    pub theta_1: f64,
}

impl fmt::Display for Factors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "theta_0 : {}\ntheta_1 : {}", self.theta_0, self.theta_1)
    }
}
