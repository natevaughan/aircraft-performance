use aircraft_performance::Scaled;
use aircraft_performance::Calculable;
use aircraft_performance::Calculation;
use aircraft_performance::search_for_nearest_curves;
use aircraft_performance::interpolate_linear;
use aircraft_performance::scale;

impl Calculable for Line {
    fn calc(&self, x: f64) -> f64 {
        lin(self.a, self.b, x)
    }
}


pub struct LinearCalculation {
    pub input_name: String,
    pub lines: Vec<Line>
}


impl Calculation for LinearCalculation {
    fn input_name(&self) -> String {
        self.input_name.to_owned()
    }

    fn calculate(&self, input: f64, scalar: f64) -> f64 {
        let (first, second) = search_for_nearest_curves(&self.lines, scalar);
        let high = first.calc(input);
        let low = second.calc(input);
        return interpolate_linear(high, low, scale(first.scalar(), second.scalar(), scalar));
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