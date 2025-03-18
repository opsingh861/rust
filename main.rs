// Lifetime

fn main() {
    let ans: &str;

    let s1 = String::from("small");
    {
        let s2 = String::from("longer");
        ans = longest(&s1, &s2); // it will not compile as lifetime of s2 is not long enough
    }

    println!("{}", ans); // if i will not access this line it will work, but here we are trying to access ans which can point to s2 which will be out of scope before this line.
}

// fn longest(s1: &str, s2: &str) -> String {
//     // this will work but when we will change the return type to &str, the problem will start
//     if s1.len() > s2.len() {
//         return s1.to_string();
//     }

//     return s2.to_string();
// }

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // needs to give lifetime specifier
    if s1.len() > s2.len() {
        return s1;
    }

    return s2;
}
