// String and Slices

fn main() {
    // let mut name = String::from("Aditya Dhanraj"); // `name` is a mutable `String` on the heap.

    // let first_name = &name[0..6]; // Borrowing `name`, creating a string slice (&str).
    // // This means `first_name` is an immutable reference to part of `name`.

    // name = String::from("John Doe"); // ERROR: Cannot mutate `name` while `first_name` is still in use.
    // // `first_name` still holds a reference to part of `name`, so Rust does not allow modifying `name`.
    // // If `first_name` is not used after this point, Rust will allow reassignment.

    // println!("{}", first_name); // This line accesses `first_name`, meaning its borrow is still active.
    // println!("{}", name); // Rust does not allow mutation while an immutable reference exists.

    // // --------- Borrowing in a function -----------

    // let first_name = get_first_name(&name); // `get_first_name` borrows `name`, but only inside the function.
    // // The borrow exists *only* for the duration of `get_first_name` and is not active after returning.

    // print!("{}", first_name); // This is allowed because `get_first_name`'s borrow has ended.

    // name = String::from("John doe"); // Allowed here because there are no **active** borrows of `name` at this point.
    let some_string = "Some data"; // String slices
    let num = [1, 2, 3, 4];
    let data = &num[0..2];
    println!("{:?}", data);
}

// Function to extract the first name from a given string
fn get_first_name(name: &String) -> &str {
    let mut index = 0; // Index to track the position of the first space

    for i in name.chars() {
        if i == ' ' {
            // Stop when the first space is found
            break;
        } else {
            index += 1; // Otherwise, increment index
        }
    }

    return &name[0..index]; // Return a slice of `name` from index 0 to the first space.
    // This slice is valid as long as `name` is still in scope and unchanged.
}
