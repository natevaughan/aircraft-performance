use serde::{Deserialize, Serialize};
pub trait Scaled {
    fn scalar(&self) -> f64;
}

pub trait Calculable {
    fn calc(&self, x: f64) -> f64;
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
