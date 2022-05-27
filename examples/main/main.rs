use emwa_rs::*;

fn main() {
    println!("adding datapoints to a regular timeseries:");
    let mut risk = EMWA::new(1 as f64, Smoothing::Static);
    for i in 1..100 {
        risk.add(i as f64).unwrap();
    }

    println!("value: {}", risk.value());

    println!("adding datapoints to a irregular timeseries (dynamic smoothing):");

    let mut observations = EMWA::new(0.5f64, Smoothing::Dynamic);

    let mut clock = 100000; // some arbitrary clock

    for i in 1..100 {
        observations
            .add_with_time(i as f64, clock as f64)
            .expect("failed to add datapoint");

        clock += 1;
    }

    println!("observations emwa: {}", observations.value());
}
