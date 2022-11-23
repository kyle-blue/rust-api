#[allow(unused_imports)]
use rand;
use std::collections::HashMap;
use std::ops::Add;

fn add<T>(x: &T, y: &T) -> T
where
    &T: Add<&T>,
{
    x + y
}

fn main() {
    const x: f32 = 5.3;
    const y: f32 = 0.2;
    println!("5.3 + 0.2 = {}", add(&x, &y));
    // println!("5.3 + 0.2 = {}", add(x, y));

    let mut _str1 = String::from("Some string");
    let mut _str2 = _str1;

    // Will not run unless you str1.clone() into str2. str1 has been moved
    // println!("{_str1}");

    let mut my_map = HashMap::<&str, &str>::new();
    my_map.insert("small", "Kevin Hart");
    my_map.insert("big", "The Rock");

    my_map = HashMap::from([
        ("small", "weiner"),
        ("big", "douchebag"),
        ("medium", "fries"),
    ]);

    for (k, v) in &my_map {
        println!("{k}, {v}");
    }
}
