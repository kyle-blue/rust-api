#[allow(unused_imports)]
use rand;
fn main() {
    let _dumb_vec = &Vec::<i32>::new();
    let mut numbers = std::vec![0, 1, 2, 3];

    numbers.push(5);

    const TEST_INDEX: u32 = 10;
    match numbers.get(TEST_INDEX as usize) {
        Some(x) => println!("{x} exists!"),
        None => println!("It doesn't exist mate"),
    }

    for val in &mut numbers {
        *val *= 2;
    }

    for (i, val) in numbers.iter().enumerate() {
        println!("{i}. {val}")
    }
}
