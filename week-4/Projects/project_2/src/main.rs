use std::io;

fn main() {
    println!("Is the employee experienced? (y/n): ");
    let mut experience_input = String::new();
    io::stdin()
        .read_line(&mut experience_input)
        .expect("Failed to read line");
    let is_experienced = experience_input.trim().to_lowercase() == "y";

    println!("Enter the employee's age: ");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    let incentive: u32 = if !is_experienced {
        100_000
    } else if age >= 40 {
        1_560_000
    } else if age >= 30 {
        1_480_000
    } else {
        1_300_000
    };

    println!("Annual incentive: N{}", incentive);
}