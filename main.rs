// structs

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("opsingh861@gmail.com"),
        username: String::from("opsingh"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1 email: {}", user1.email);

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Red color: {}, {}, {}", red.0, red.1, red.2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of rect1: {}", rect1.area());
    println!(
        "Can rect1 hold rect2: {}",
        rect1.can_hold(&Rectangle {
            width: 10,
            height: 40
        })
    );

    let square = Rectangle::square(10);
    println!("Area of square: {}", square.area());
}
