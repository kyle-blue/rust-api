use rand;
use rand::Rng;
#[allow(unused_imports)]
use std::io;

fn main() {
    const LOWER: i32 = 0;
    const HIGHER: i32 = 5;
    println!("Guess the number in the range {LOWER}-{HIGHER}");

    let _unused = "bruh";
    let random_num = rand::thread_rng().gen_range(LOWER..=HIGHER);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read from stdin");

    guess = guess.trim_end().to_owned();

    let result = if guess.parse::<i32>().expect("Could not parse input") == random_num {
        "correct"
    } else {
        "incorrect"
    };

    println!("The number to guess is {random_num}. You guessed {guess}. You are {result}")
}
