#[allow(unused_imports)]
use rand;
fn main() {
    let mut tup = ("Kyle".to_owned(), 23, "Male");

    println!("Name is {}", tup.0);

    tup.1 = 132;

    // Destructuring tuple
    let (_name, age, _gender) = tup;
    println!("Age is {age}")
}
