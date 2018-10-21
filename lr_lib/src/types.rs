use std::fmt;

#[derive(Deserialize)]
pub struct ParsingData {
    pub mileage: f64,
    pub price: f64,
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
