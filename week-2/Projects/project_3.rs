fn main(){
    let p: f64 = 210_000.0;
    let n: i32 = 3; 
    let r: f64 = 5.0; 
    let a = p * (1.0 - (r / 100.0)).powi(n);
    println!("Over the span of 3 years, this TV has depreciated to {:.2} naira", a);
}
