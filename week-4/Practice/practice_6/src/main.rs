use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

println!("Input lower bound");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lowerbound:i32 = input1.trim().parse().expect("Failed to input");
    println!("Input upper bound");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upperbound:i32 = input2.trim().parse().expect("Failed to input");
    let trueupperbound:i32 = upperbound - 1; // yes this might look like oversabi, but I want to show what I'm counting 
    // and not juss count, so I create a variable that says the true upper bound, remove 1 and I have the number I intend 
    // to use
println!("Counting from {} to {}", lowerbound, trueupperbound); // so that I'd be shown e.g 30, and not 31, given I ain't 
// counting 31
    for x in lowerbound..upperbound{
        println!("Number {}", x);
    }
}
