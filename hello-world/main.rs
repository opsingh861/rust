fn main() {
    // Variables are immutable by default in Rust
    // let x = 5; // it is immutable
    // x = 6; // error: cannot assign twice to immutable variable `x`

    let mut x = 5; // it is mutable
    println!("The value of x is: {}", x);
    x = 6; // no error because x is mutable
    println!("The value of x is: {}", x);
}
