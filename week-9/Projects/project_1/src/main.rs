use std::io::Write;
use std::io::Read;
fn main() {
    let header = "Lager, Stout, Non-alcoholic";
    let data_1 = "\n33 Export, Legend, Maltina";
    let data_2 = "\nDesperados, Turbo King, Amstel Malta";
    let data_3 = "\nGoldberg, Williams, Malta Gold";
    let data_4 = "\nGulder, _, Fayrouz";
    let data_5 = "\nHeineken, _, _";
    let data_6 = "\nStar, _, _";
    let mut file = std::fs::File::create("breweries.csv").expect("Failed to create");
    file.write_all(header.as_bytes()).expect("Failed to write into file");
    file.write_all(data_1.as_bytes()).expect("Failed to write into file");
    file.write_all(data_2.as_bytes()).expect("Failed to write into file");
    file.write_all(data_3.as_bytes()).expect("Failed to write into file");
    file.write_all(data_4.as_bytes()).expect("Failed to write into file");
    file.write_all(data_5.as_bytes()).expect("Failed to write into file");
    file.write_all(data_6.as_bytes()).expect("Failed to write into file");

    let mut opened_file = std::fs::File::open("breweries.csv").expect("Failed to open file");

    let mut contents = String::new();
    opened_file.read_to_string(&mut contents).expect("Failed to read the file");
    println!("{}", contents);
}