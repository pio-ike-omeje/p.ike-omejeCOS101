struct Laptop {
    brand: String,
    price: u32,
}
impl Laptop {
    // Method to calculate cost for a given purchase quantity
    fn purchase_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}
fn main() {
    // Instantiate laptops with prices (in Naira)
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };
    let purchase_qty = 3;
    // Calculate individual costs using the method
    let hp_cost = hp.purchase_cost(purchase_qty);
    let ibm_cost = ibm.purchase_cost(purchase_qty);
    let toshiba_cost = toshiba.purchase_cost(purchase_qty);
    let dell_cost = dell.purchase_cost(purchase_qty);
    // Sum total cost across all four brands
    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;
    println!("Cost for 3 {} laptops: ₦{}", hp.brand, hp_cost);
    println!("Cost for 3 {} laptops: ₦{}", ibm.brand, ibm_cost);
    println!("Cost for 3 {} laptops: ₦{}", toshiba.brand, toshiba_cost);
    println!("Cost for 3 {} laptops: ₦{}", dell.brand, dell_cost);
    println!("\nTotal cost for all purchases: ₦{}", total_cost);
}