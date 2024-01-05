use serde::{Deserialize, Serialize};
pub trait Scaled {
    fn scalar(&self) -> f64;
}

pub struct Performance {
    pub calculations: Vec<&'static dyn Calculation>
}

pub trait Calculable {
    fn calc(&self, x: f64) -> f64;
}

pub trait Calculation {
    fn input_name(&self) -> String;
    fn calculate(&self, input: f64, scalar: f64) -> f64;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Criteria {
    pub pressure_alt: f64,
    pub temp_c: f64,
    pub take_off_weight: f64,
    pub headwind: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceData {
    pub ground_roll: i64,
    pub vr: i64,
}

pub fn search_for_nearest_curves<T: Scaled>(curves: &[T], scalar: f64) -> (&T, &T) {

    if curves.len() < 2 {
        panic!("Cannot interpolate fewer than 2 weight_curves")
    }

    let mut smallest = &curves[0];
    let mut second_smallest = &curves[1];
    for x in curves {
        if (scalar - x.scalar()).abs() < (scalar - smallest.scalar()).abs() {
            second_smallest = smallest;
            smallest = &x;
        }
    }
    (smallest, second_smallest)
}


pub fn interpolate_linear(first: f64, second: f64, scalar: f64) -> f64 {
    (second - first) * scalar + first
}

pub fn scale(first: f64, second: f64, val: f64) -> f64 {
    (val - first) / (second - first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct TestCurve {
        pub scalar: f64
    }

    impl Scaled for TestCurve {
        fn scalar(&self) -> f64 {
            self.scalar
        }
    }


    const SMALLEST: TestCurve = TestCurve { scalar: -3.0 };
    const MIDDLE: TestCurve = TestCurve { scalar: -2.0 };
    const LARGEST: TestCurve = TestCurve { scalar: 4.0 };
    const X_LARGEST: TestCurve = TestCurve { scalar: 400.0 };

    #[test]
    fn test_search_for_nearest_curves_order() {
        let ordered = [SMALLEST, MIDDLE, LARGEST];
        let (first, second) = search_for_nearest_curves(&ordered, -6.0);
        assert_eq!(first, &SMALLEST);
        assert_eq!(second, &MIDDLE);
    }

    #[test]
    fn test_search_for_nearest_curves_two_in_wrong_order() {
        let ordered = [LARGEST, SMALLEST];
        let (first, second) = search_for_nearest_curves(&ordered, -6.0);
        assert_eq!(first, &SMALLEST);
        assert_eq!(second, &LARGEST);
    }

    #[test]
    fn test_search_for_nearest_curves_unordered() {
        let ordered = [MIDDLE, LARGEST, SMALLEST, X_LARGEST];
        let (first, second) = search_for_nearest_curves(&ordered, -6.0);
        assert_eq!(first, &SMALLEST);
        assert_eq!(second, &MIDDLE);
    }

    #[test]
    fn test_search_for_nearest_curves_reverse() {
        let ordered = [X_LARGEST, LARGEST, MIDDLE, SMALLEST];
        let (first, second) = search_for_nearest_curves(&ordered, -6.0);
        assert_eq!(first, &SMALLEST);
        assert_eq!(second, &MIDDLE);
    }
    
    #[test]
    fn test_scale_in_range() {
        assert_eq!(scale(5.0, 10.0, 7.5), 0.5);
    }

    #[test]
    fn test_scale_out_of_range() {
        assert_eq!(scale(5.0, 10.0, 2.5), -0.5);
    }
}