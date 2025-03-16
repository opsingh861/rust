// hashmaps

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("Aditya Dhanraj", 24);
    map.insert("John", 20);

    println!("{:?}", map);

    let tuple = vec![('a', 1), ('b', 2)];

    let key_value_pair = get_map(tuple);

    println!("{:?}", key_value_pair)
}

fn get_map(tuple: Vec<(char, i32)>) -> HashMap<char, i32> {
    let mut some_map = HashMap::new();

    for (c, n) in tuple {
        some_map.insert(c, n);
    }

    return some_map;
}
