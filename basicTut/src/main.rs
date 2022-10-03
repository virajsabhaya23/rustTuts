//std is a crate(package) and io is a module
// :: ~ path separator in rust (goes from package to module/ module to function)
use std::io;

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

    //this is a constant
    const MAX_POINTS: u32 = 100;
    println!("the value of MAX_POINTS is: {}", MAX_POINTS);

    /*  DATA TYPES
    1. Scalar Types
        - integers (i are signed and u are unsigned)
            by default integers are i32 (ranging from i8 to i128)
        - Floating-Point Types (f32 and f64)
        - boolean (true or false)(0 or 1)
        - characters (let Fname_initial: char = 'A';)
    2. Compound Types
        - Tuples
    */
    let tup: (i32,bool,char)=(1,true,'s');
    println!("{}", tup.2);
    /*
        - Arrays (let a:[i32:5] = [1,2,3,4,5];)
    */


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input);
}