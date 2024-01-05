use aircraft_performance::Scaled;
use aircraft_performance::Calculable;

impl Calculable for Line {
    fn calc(&self, x: f64) -> f64 {
        lin(self.a, self.b, x)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Line {
    pub scalar: f64,
    pub a: f64,
    pub b: f64,
}

impl Scaled for Line {
    fn scalar(&self) -> f64 {
        self.scalar
    }
}

pub fn lin(a: f64, b: f64, x: f64) -> f64 {
    a * x + b
}