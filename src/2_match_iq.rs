#[allow(unused_imports)]
use rand;
fn main() {
    const my_iq: i32 = 101;
    const mensa_iq: i32 = 140;

    match my_iq {
        100 => println!("Dead on average"),
        90..=110 => println!("Pretty average iq bucko"),
        0..=89 => println!("DUmBasS"),
        111..=i32::MAX => println!("Smartass"),
        _ => println!("How tf do you have a negative iq?"),
    }

    match my_iq.cmp(&mensa_iq) {
        std::cmp::Ordering::Greater => println!("Welcome to mensa"),
        std::cmp::Ordering::Less => println!("Get the fuck out of mensa you cretin"),
        std::cmp::Ordering::Equal => {
            println!("You are just about in mensa bucko. I'm watching you...")
        }
        _ => println!("THis makes no sense"),
    }
}
