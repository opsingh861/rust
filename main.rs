// Options are an enum type that is defined in the standard library, since there is not concept of null here so we can use option to return None when we want to return nothing.

// pub enum Option<T> { // Option is a generic enum type with one type parameter
//     None,
//     Some(T),
// }

fn main() {
    let my_name = String::from("Rust");
    let result = first_a_index(&my_name);
    match result {
        Some(index) => println!("The first 'a' in 'Rust' is at index: {}", index),
        None => println!("There is no 'a' in {}", my_name),
    }
}

fn first_a_index(s: &str) -> Option<usize> {
    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(i);
        }
    }
    None
}
