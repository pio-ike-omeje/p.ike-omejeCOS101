fn main(){
    let v = vec![20, 40, 60, 80];
    // vector v owns the object in heap
    
    let v2 = v; // ownership has been passed from v to v2
    let v2_return = display(v2);// ownership has been passed from v2 to display and then to v2_return
    println!("In main {:?}",v);
}

fn display(v:Vec<i32>)->Vec<i32> {
    // returning same vector
    println!("inside display {:?}",v);
    return v;
}
