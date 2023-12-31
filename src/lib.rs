pub trait Scaled {
    fn scalar(&self) -> f64;
}

pub trait Calculable {
    fn calc(&self, x: f64) -> f64;
}