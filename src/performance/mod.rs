use aircraft_performance::Calculable;
use aircraft_performance::Calculation;
use aircraft_performance::Criteria;
use aircraft_performance::PerformanceData;
use aircraft_performance::Scaled;
use aircraft_performance::search_for_nearest_curves;
use aircraft_performance::scale;
use aircraft_performance::interpolate_linear;
use linear::lin;
use linear::Line;
use quadratic::QuadCurve;

use self::quadratic::QuadCalculation;

mod linear;
mod quadratic;

pub fn calculate(c: Criteria) -> PerformanceData {
    let iso_temp_f = lin(1.8, 32.0, c.temp_c);

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


    let c1 = QuadCalculation {
        input_name: "temp_f".to_string(),
        quad_curves: Vec::from(curves)
    };

    let init_roll = c1.calculate(iso_temp_f, c.pressure_alt);
    let weight_scalar = scale(2000.0, 2750.0, c.take_off_weight);
    let vr = interpolate_linear(60.0, 71.0, weight_scalar);

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
    let val3 = first.calc(c.take_off_weight);
    let val4 = second.calc(c.take_off_weight);
    let adj_roll = interpolate_linear(val3, val4, scale(first.scalar(), second.scalar(), init_roll));

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
    let val5 = first_l.calc(c.headwind);
    let val6 = second_l.calc(c.headwind);
    let final_roll = interpolate_linear(val5, val6, scale(first_l.scalar(), second_l.scalar(), adj_roll));
    PerformanceData {
        ground_roll: final_roll.round() as i64,
        vr: vr.round() as i64
    }
}