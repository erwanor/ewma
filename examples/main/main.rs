use emwa_rs::EMWA;

fn main() {
    println!("this is a test");
    let mut risk = EMWA::new(0.4);
    for i in 1..100 {
        risk.add(i as f64);
    }
    println!("value: {}", risk.value())
}
