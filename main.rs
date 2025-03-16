// Error handling
use std::fs::read_to_string;

// enum Result<T, E> { // Result is a generic type with two type parameters: T and E (T is the type of the value that the Result type is holding, and E is the type of the error that the Result type is holding)
//     Ok(T),
//     Err(E),
// }

fn main() {
    // let result = fs::read("hello.txt");
    // match result {
    //     Ok(content) => {
    //         println!("File content: {:?}", content);
    //     }
    //     Err(error) => {
    //         println!("Error: {:?}", error);
    //     }
    // }

    // let content = fs::read("hello.txt").unwrap(); // unwrap() returns the value inside the Ok variant if the Result is Ok, and panics if the Result is Err
    // println!("File content: {:?}", content); // This line will not be executed if the Result is Err because unwrap() will panic

    let content = read_file("hello.txt".to_string());
    print!("File content: {:?}", content);
    print!("End of program");
}

fn read_file(file_name: String) -> Result<String, String> {
    let result = read_to_string(file_name);
    match result {
        Ok(content) => Ok(content),
        Err(_) => Err("Error reading file".to_string()),
    }
}
