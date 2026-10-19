use std::io::Read;
fn main(){

    let mut file = std::fs::File::open("data.txt").expect("Failed to open.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read. ");
    println!("{}", contents );
}
