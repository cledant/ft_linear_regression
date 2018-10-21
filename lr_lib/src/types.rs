#[derive(Deserialize)]
pub struct ParsingData {
    pub mileage: f64,
    pub price: f64,
}

#[derive(Clone)]
pub struct Factors {
    pub theta_0: f64,
    pub theta_1: f64,
}

pub static LEARNING_RATE: f64 = 0.01;
