// traits

use std::iter::Sum;

pub trait Summary {
    // fn summarize(&self) -> String;

    fn summarize(&self) -> String {
        // self parameter should be there
        // we can define default implementation also, if we implement it again at other place so it will overridden
        return format!("This is default summarize");
    }
}

pub trait Fix {
    fn console(&self) {
        println!("This is from Fix");
    }
}

struct User {
    name: String,
    age: i32,
}

// impl Summary for User {
//     fn summarize(&self) -> String {
//         return format!("My name is {} and my age is {}", self.name, self.age);
//     }
// }

impl Summary for User {}
// can implement more than one
impl Fix for User {}

fn main() {
    let user = User {
        name: String::from("Aditya Dhanraj"),
        age: 24,
    };

    // println!("{}", user.summarize());

    notify(user);
}

// fn notify(u: impl Summary) {
//     // u should implement Summary trait
//     println!("{}", u.summarize())
// }

// pub fn nofify2<T: Summary>(u: T) {
//     // this is same as above, above is syntexial sugar
//     println!("{}", u.summarize())
// }

// fn notify(u: impl Summary + Fix) {
//     u.console();
//     u.summarize();
// }

pub fn notify<T: Summary + Fix>(u: T) {
    // both are same the above one and this one
    u.console();
    u.summarize();
}
