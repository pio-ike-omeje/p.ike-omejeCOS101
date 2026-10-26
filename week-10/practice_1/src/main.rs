fn main() {
    let v = vec![101, 250, 330, 400];

    /* vector v owns the value as it's stored on the heap, and only
    a single variablle can own the value at a time 

    */

    let v2 = v;
    /* here 2 variables own heap value*/

    println!("{:?}", v);
}


