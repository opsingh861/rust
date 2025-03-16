// enums

enum Direction {
    East,
    West,
    North,
    South,
}

enum Shapes {
    Circle(u32),
    Square(u32),
    Rectangle(u32, u32),
}

fn calculate_area(shape: Shapes) -> f64 {
    // pattern matching
    match shape {
        Shapes::Circle(radius) => 3.14 * (radius as f64) * (radius as f64),
        Shapes::Rectangle(length, breadth) => (length as f64) * (breadth as f64),
        Shapes::Square(side) => (side as f64) * (side as f64),
    }
}

fn main() {
    // let some_direction = Direction::East;
    // let wrong_direction = Direction::Something; // Error: no variant named `Something` found for type `Direction` in the current scope

    let circle = Shapes::Circle(10);
    let another_shape = Shapes::Rectangle(10, 20);
    let rectangle = Shapes::Rectangle(10, 20);

    println!("Area of circle: {}", calculate_area(circle));
}
