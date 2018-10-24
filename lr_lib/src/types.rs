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
	pub ms_error: f64,
	pub stop_iter: usize,
}

impl fmt::Display for Factors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Theta_0 : {}\nTheta_1 : {}\nMean Squared Error : {}%\nEnd Iter : {}",
			self.theta_0, self.theta_1, self.ms_error * 100.0, self.stop_iter)
    }
}
