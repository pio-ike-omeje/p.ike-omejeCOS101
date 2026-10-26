//declare dimensions of a rectangle
struct Rectangle {
    width:u32, height:u32
}
//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self)->u32 {
        //use the . operator to fetch the value of a field using the self keyword
        self.width * self.height
    }
}
fn main() {
    // instanatiate the structure
    let test_example = Rectangle {
        width:10,
        height:20
    };
    //print the rectangle's area along side it's width and height 
    println!("width is {}, height is {}, area of Rectangle is {}",test_example.width,test_example.height,test_example.area());
}