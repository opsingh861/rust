// variables and data types

fn main() {
    let x: i32 = -5; // signed 32-bit integer
    let y: u32 = 5; // unsigned 32-bit integer
    let z: f32 = 5.0; // 32-bit floating point number

    println!("x: {} y: {} z: {}", x, y, z);

    let a = 5; // Rust can infer the type of a variable from its value

    let is_male = true; // boolean
    let is_above_18 = false; // boolean

    let c = 'a'; // character
    let d = "Hello, World!"; // string (&str)
    let greeting = String::from("Hello, World!"); // string (String struc)
    let char1 = greeting.chars().nth(0); // get the first character of the string as an Option type (Option<char>)
    print!("c: {}", char1.unwrap()); // unwrap is used to get the value from an Option type, it will panic if the value is None
}
