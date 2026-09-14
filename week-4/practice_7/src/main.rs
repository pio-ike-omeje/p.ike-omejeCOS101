use std::io;

fn main() {
    let mut input = String::new();

    println!("Input the number");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num:i32 = input.trim().parse().expect("Failed to input");

    while num < 10 {
        println!("Inside loop number value is {}", num);
            num+=1;
            
    } 
    println!("Outside loop number value is {}",num );
}
