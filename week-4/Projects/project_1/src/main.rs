use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Input the coefficient of the x squared in the quadratic equation.");
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a:f64 = input_a.trim().parse().expect("Failed to input");

    println!("Input the coefficient of x the quadratic equation.");
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b:f64 = input_b.trim().parse().expect("Failed to input");

    println!("Input the constant of the quadratic equation.");
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c:f64 = input_c.trim().parse().expect("Failed to input");

// Calculate d
let d = b * b - 4.0 * a *c;
// Determine roots based on the value of d
if d > 0.0 {
    let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("The two roots are : {} and {}", root1, root2);
} else if d == 0.0 {
    let root = -b / (2.0 * a);
    println!("Exactly one root: {}", root );
} else {
    println!("No real roots");
}
}
