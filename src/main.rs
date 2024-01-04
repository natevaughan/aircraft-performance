use aircraft_performance::Criteria;
use crate::performance::calculate;
use crate::prisma::client;
use std::env;
use crate::prisma::model::aircraft;

mod performance;
#[allow(warnings, unused)]
mod prisma;


#[tokio::main]
async fn main() {
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
    });
    let client = client().await;
    let all_aircraft = client
                .aircraft()
                .find_many(vec![])
                .with(aircraft::manufacturer::fetch())
                .exec()
                .await
                .unwrap();

    for ac in all_aircraft {
        println!("Available aircraft: {} {}", ac.manufacturer.unwrap().name, ac.name);
    }
    println!("Ground roll: {:>5}", performance.ground_roll);
    println!("Vr:          {:>5}", performance.vr);
}