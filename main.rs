// Vectors

fn main() {
    let mut numbers = Vec::new();

    numbers.push(1); // if this is commented then rust is not able to infer the type of Vector and throwing error at above line, because explicitly we have not defined the type
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);

    // let even_numbers = filter_even(numbers);

    filter_even_inplace(&mut numbers);
    println!("");
    print!("{:?}", numbers);

    print_vec(&mut numbers);
}

fn filter_even(numbers: Vec<i32>) -> Vec<i32> {
    // with extra storage
    let mut ans: Vec<i32> = Vec::new();
    for num in numbers {
        if num % 2 == 0 {
            ans.push(num);
        }
    }

    return ans;
}

fn filter_even_inplace(numbers: &mut Vec<i32>) {
    let mut i = 0;
    while i < numbers.len() {
        if numbers[i] % 2 != 0 {
            print!("{} ", numbers[i]);
            numbers.remove(i);
        } else {
            i += 1;
        }
    }
}

fn print_vec(v: &Vec<i32>) {
    println!();
    let mut new_vec: Vec<i32> = Vec::new();
    for num in v {
        new_vec.push(*num); // need to deference
    }
}
