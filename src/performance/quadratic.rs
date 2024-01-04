use aircraft_performance::Scaled;
use aircraft_performance::Calculable;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct QuadCurve {
    pub scalar: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Scaled for QuadCurve {
    fn scalar(&self) -> f64 {
        self.scalar
    }
}

impl Calculable for QuadCurve {
    fn calc(&self, x: f64) -> f64 {
        self.b * (x - self.a).powi(2) + self.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad() {
        let curve = QuadCurve {
            scalar: 1.0,
            a: -4044.0, 
            b: 0.0001636, 
            c: -7550.0
        };
        assert_eq!(curve.calc(2750.0), 1.5201295999995637);
    }
}