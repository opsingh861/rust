fn main() {
    // Ownership
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // This will not work because s1 has been moved to s2
    println!("{}, world!", s2);

    let some_string = String::from("hello");
    transfer_ownership(some_string);
    // println!("{}", some_string); // This will not work because some_string has been moved to transfer_ownership

    let mut one_more_string = String::from("hello");
    one_more_string = take_and_give_back(one_more_string);
    println!("{}", one_more_string); // This will work because one_more_string has been moved to take_and_give_back and then moved back to one_more_string

    // Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4); // This will work because s3 has been cloned to s4

    let s5 = String::from("hello");
    transfer_ownership(s5.clone());
    println!("{}", s5); // This will work because s5 has been cloned to transfer_ownership
}

fn transfer_ownership(some_string: String) {
    println!("{}", some_string); // This will work because some_string has been moved to transfer_ownership
}

fn take_and_give_back(a_string: String) -> String {
    return a_string;
}
