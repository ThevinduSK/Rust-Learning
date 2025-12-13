// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first);
//     println!("{full}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

//Cloning avoids moving ownership
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}


/*
Ownership is primarily a discipline of heap management:

All heap data must be owned by exactly one variable.
Rust deallocates heap data once its owner goes out of scope.
Ownership can be transferred by moves, which happen on assignments and function calls.
Heap data can only be accessed through its current owner, not a previous owner.

*/