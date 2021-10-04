mod digit_powers;
use digit_powers::DigitPowers;

fn main() {
    println!("Calculation started");
    let mut dps = DigitPowers::new(5);
    println!("{:?}", dps.get_numbers());
    let answer_p30: u128 = dps.get_numbers().iter().sum();

    println!(
        "The answer to problem 30 of project Euler is {}.",
        answer_p30
    );
}
