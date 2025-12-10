// Constant
const MAX_POINTS: u32 = 100_000;

fn main() {
    
    println!("The maximum points are: {MAX_POINTS}");

    // Mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Imutable variable
    let y = 10;
    println!("The value of y is: {y}");

    // Shadowing
    // by using let again we can shadow the previous variable
    // we can also change the type of the variable when we shadow it
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");

    let spaces = "   ";
    let spaces = spaces.len();


}