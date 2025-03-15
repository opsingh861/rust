fn main() {
    // Loop
    for i in 0..5 {
        // 0, 1, 2, 3, 4
        println!("i is = {}", i);
    }

    println!("Looping through string: ");

    let word = "Hello, World!";
    for c in word.chars() {
        // H, e, l, l, o, ,,  , W, o, r, l, d, !
        println!("c is = {}", c);
    }

    println!("Looping through string: ");
    let word1 = String::from("World!");
    for c in word1.chars() {
        // H, e, l, l, o, ,,  , W, o, r, l, d, !
        println!("c is = {}", c);
    }

    let mut i = 0;
    while i < 100 {
        i += 1;
        println!("i is = {}", i);
    }
}
