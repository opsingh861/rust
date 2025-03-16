// External crate or package ( cargo add package_name )
use rand::random;

fn main() {
    let random_number = random::<u128>();
    println!("Random number: {}", random_number);
}
