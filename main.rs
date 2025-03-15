fn main() {
    // Borrowing
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1; // we can have multiple immutable references to the same value
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable references
    let something = String::from("hello");
    wrong_change(&something); // This will not compile because s is borrowed as immutable
    wrong_change(&mut something); // this will also throw an error because we are trying to borrow the value as mutable which is immutable in nature

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut r1 = String::from("hello");
    let r2 = &mut r1;
    let r3 = &mut r1; // We cannot have multiple mutable references to the same value but there is catch, if we are not using the first mutable reference then we can have mutable reference to the same value
    let r4 = &r1; // It will also throw error because after trying borrowing anything as mutable reference we can't even borrow it as immutable reference
    print!("{}", r1); // Till now it will work
    print!("{}", r2); // Till now it will work as we are we are using r2 which was which was borrowed as mutable reference
    print!("{}", r3); // This will throw an error because we can't have multiple mutable references to the same value
}

fn calculate_length(s: &String) -> i32 {
    return s.len() as i32;
}

fn wrong_change(something: &String) {
    something.push_str(", world"); // we are trying to modify the borrowed value which is not mutable in nature
}

fn change(s: &mut String) {
    s.push_str(", world");
}
