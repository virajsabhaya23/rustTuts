fn main() {
    // A simple example for a  
    // muttable variable in rust
    let mut x = 5;
    println!("the value of x is: {}", x);

    x = x + 1;
    println!("the value of x is: {}", x);

    //the reason this works is because
    // we are redefining the variable x
    let x = "hello";
    println!("the value of x is: {}", x);
}
