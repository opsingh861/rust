// Display trait

use std::fmt;

struct User<'a> {
    name: &'a str, // without lifetime specifier it will give error
}

impl fmt::Display for User<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User {{ name: {} }}", self.name)
    }
}

fn main() {
    // let name = String::from("Aditya Dhanraj");

    // {
    //     let user = User { name: &name };
    // }

    // println!("{}", name); // this will work as name is not going out of scope

    let user = User {
        name: "Aditya Dhanraj",
    };

    println!("{}", user); // this will not work
}
