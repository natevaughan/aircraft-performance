use std::env;
use crate::linear::scale;
use crate::linear::interpolate_linear;
use crate::linear::lin;
use crate::quadratic::quad;

mod linear;
mod quadratic;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: cargo run <outside_air_temp> <pressure_alt> <take_off_weight> <headwind>");
        return
    }
    let iso_temp_c: f64 = args[1].parse().expect("Please provide a numeric value for ISO temp");
    let iso_temp_f = lin(1.8, 32.0, iso_temp_c);
    let pressure_alt: f64 = args[2].parse().expect("Please provide a numeric value for pressure altitude");
    let take_off_weight: f64 = args[3].parse().expect("Please provide a numeric value for takeoff weight");
    let wind: f64 = args[4].parse().expect("Please provide a numeric value for wind weight");

    // First calc
    let curves = [QuadCurve {
        scalar: 0.0,
        a: -184.484, 
        b: 0.024538,
        c: 283.3
    }, QuadCurve {
        scalar: 2000.0,
        a: -148.028, 
        b: 0.0362252,
        c: 665.469
    }, QuadCurve {
        scalar: 4000.0,
        a: -216.325, 
        b: 0.0318901,
        c: 392.791
    }, QuadCurve {
        scalar: 6000.0,
        a: -235.315, 
        b: 0.0353866,
        c: 434.767
    }, QuadCurve {
        scalar: 7000.0,
        a: -175.482, 
        b: 0.051501,
        c: 1064.07
    }];

    let (nearest_low, nearest_high) = search_for_nearest_curves(&curves, pressure_alt);
    println!("Nearest two are {} and {}", nearest_low.scalar, nearest_high.scalar);

    let val1 = calc_from_curve(nearest_low, iso_temp_f);
    let val2 = calc_from_curve(nearest_high, iso_temp_f);
    let init_roll = interpolate_linear(val1, val2, scale(nearest_low.scalar, nearest_high.scalar, pressure_alt));
    let weight_scalar = scale(2000.0, 2750.0, take_off_weight);
    let vr = interpolate_linear(60.0, 71.0, weight_scalar);
    println!("Initial roll: {:>5}", init_roll.round() as i64);

    // Second calc
    let weight_curves = [QuadCurve {
        scalar: 3300.0,
        a: -4043.7, 
        b: 0.000163585,
        c: -4250.13
    }, QuadCurve {
        scalar: 3000.0,
        a: -473.892, 
        b: 0.000325273,
        c: -380.715
    }, QuadCurve {
        scalar: 2500.0,
        a: 25.0, 
        b: 0.0003333333,
        c: 24.7917
    }, QuadCurve {
        scalar: 2050.0,
        a: 383.947, 
        b: 0.000324786,
        c: 231.78
    }, QuadCurve {
        scalar: 1710.0,
        a: -379.118, 
        b: 0.000203106,
        c: -278.691
    }];

    let (first, second) = search_for_nearest_curves(&weight_curves, init_roll);
    println!("Nearest two are {} and {}", first.scalar, second.scalar);
    let val3 = calc_from_curve(first, take_off_weight);
    let val4 = calc_from_curve(second, take_off_weight);
    let adj_roll = interpolate_linear(val3, val4, scale(first.scalar, second.scalar, init_roll));
    println!("adjusted for weight roll: {:>5}", adj_roll.round() as i64);

    // Third calc
    let lines = [Line {
        scalar: 1275.0,
        a: -17.0, 
        b: 1275.0,
    }, Line {
        scalar: 1725.0,
        a: -22.0, 
        b: 1725.0,
    }, Line {
        scalar: 2240.0,
        a: -26.0, 
        b: 2240.0,
    }, Line {
        scalar: 2750.0,
        a: -34.0, 
        b: 2750.0,
    }, Line {
        scalar: 3350.0,
        a: -40.0, 
        b: 3350.0,
    }];

    let (first_l, second_l) = search_for_nearest_curves(&lines, adj_roll);
    println!("Nearest two are {} and {}", first_l.scalar, second_l.scalar);
    let val5 = calc_from_line(first_l, wind);
    let val6 = calc_from_line(second_l, wind);
    let final_roll = interpolate_linear(val5, val6, scale(first_l.scalar, second_l.scalar, adj_roll));
    println!("final roll: {:>5}", final_roll.round() as i64);
    println!("Vr:         {:>5}", vr.round() as i64);
}

fn calc_from_line(c: &Line, x: f64) -> f64 {
    lin(c.a, c.b, x)
}

fn calc_from_curve(c: &QuadCurve, x: f64) -> f64 {
    quad(c.a, c.b, c.c, x)
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

struct QuadCurve {
    name: String,
    scalar: f64,
    a: f64,
    b: f64,
    c: f64,
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