use std::env;
use aircraft_performance::Scaled;
use crate::performance::calculate;
use crate::performance::Criteria;

mod performance;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: aircraft-performance <outside_air_temp> <pressure_alt> <take_off_weight> <headwind>");
        return
    }
    let iso_temp_c: f64 = args[1].parse().expect("Please provide a numeric value for ISO temp");
    let pressure_alt: f64 = args[2].parse().expect("Please provide a numeric value for pressure altitude");
    let take_off_weight: f64 = args[3].parse().expect("Please provide a numeric value for takeoff weight");
    let wind: f64 = args[4].parse().expect("Please provide a numeric value for wind weight");
    let performance = calculate(Criteria {
        temp_c: iso_temp_c,
        pressure_alt: pressure_alt,
        take_off_weight: take_off_weight,
        headwind: wind
    })
    println!("Ground roll: {:>5}", performance.ground_roll);
    println!("Vr:          {:>5}", performance.vr);
}

fn search_for_nearest_curves<T: Scaled>(curves: &[T], scalar: f64) -> (&T, &T) {

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

#[cfg(test)]
mod tests {
    use super::*;

    const SMALLEST: QuadCurve = QuadCurve { scalar: -3.0, a: 1.0, b: 1.0, c: 1.0 };
    const MIDDLE: QuadCurve = QuadCurve { scalar: -2.0, a: 1.0, b: 1.0, c: 1.0 };
    const LARGEST: QuadCurve = QuadCurve { scalar: 4.0, a: 1.0, b: 1.0, c: 1.0 };
    const X_LARGEST: QuadCurve = QuadCurve { scalar: 400.0, a: 1.0, b: 1.0, c: 1.0 };

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
}