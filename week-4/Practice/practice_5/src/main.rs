use std::io;

fn main() {
    let mut input = String::new();

    println!("\n Yo enter your height in centimeters");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0 {
        println!("Yo you're pretty average dude");

    } else if height > 170.0 && height <= 195.0 {
        println!("You are tall man!");
    } else if height < 150.0 && height > 100.0 {
        println!("You are short dude!");
    } else {
        println!("OK your height is pretty weird" );
    }
}
