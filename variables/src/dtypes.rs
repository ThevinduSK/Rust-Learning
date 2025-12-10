fn main() {
    // Integer types
    let x: i32 = 42;
    let y: u32 = 100;
    
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
    
    // Floating point types
    let a: f64 = 3.14;
    let b: f32 = 2.5;
    
    println!("f64: {}", a);
    println!("f32: {}", b);
    
    // Boolean
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);
    
    // Character
    let letter: char = 'Z';
    let emoji: char = '🦀';
    
    println!("Character: {}", letter);
    println!("Emoji: {}", emoji);
    
    // Tuple
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    println!("Tuple: {:?}", tuple);

    let (x,y,z) = tuple;
    println!("Tuple values: {}, {}, {}", x, y, z);

    let first_element = tuple.0;
    println!("First element of tuple: {}", first_element);
    
    // Tuple without any value special name, unit
    // This value and its corresponding type are both written () 
    // and represent an empty value or an empty return type
    // Expressions implicitly return the unit value 
    // if they don’t return any other value.

    // we can modify individual elements of a mutable tuple

    // Array - Same type, fixed length
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("Months: {:?}", months);


}
