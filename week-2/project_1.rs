fn main() {
    let p: f64 = 520_000_000.0;
    let n: i32 = 5;
    let r: f64 = 10.0;

    // Calculate total amount
    let a = p * (1.0 + (r / 100.0)).powi(n);

    // Calculate the compound interest
    let c = a - p;

    println!("Compound interest: N{:.2}", c);
}
