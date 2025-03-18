// Lifetime of structs

struct User<'a> {
    name: &'a str, // without lifetime specifier it will give error
}

fn main() {
    // let name = String::from("Aditya Dhanraj");

    // {
    //     let user = User { name: &name };
    // }

    // println!("{}", name); // this will work as name is not going out of scope

    let user;

    {
        let name = String::from("Aditya Dhanraj");
        user = User { name: &name } // same issue, name will be out of scope and we are trying to access the it out of the scope, problem is with the reference. If that values itself will be not there how can use that.
    }

    println!("{:?}", user.name); // this will not work
}
