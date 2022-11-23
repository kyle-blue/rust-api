#[allow(unused_imports)]
use rand;
fn main() {
    let arr = [0, 1, 2, 3, 4, 5];

    for val in arr.iter() {
        println!("Value is {val}")
    }

    // Reversing an iterator
    for val in (1..=10).rev() {
        println!("{val}")
    }

    let mut i: usize = 0;
    loop {
        println!("I would loop forever without a break...");
        if i == 5 {
            break;
        }
        i += 1;
    }
}
