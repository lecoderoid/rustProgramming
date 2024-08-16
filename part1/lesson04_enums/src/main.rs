// #![allow(dead_code)]
// mod mytypes;
// use mytypes::Color;

mod mytypes_for_demo_purposes;
use mytypes_for_demo_purposes::{Color, HouseLocation};

fn main() {
    // demo_simple_enums();
    // demo_with_data();
    demo_using_option_enum();
}

fn demo_simple_enums() {
    println!("Demo simple enums");

    let c: Color = Color::Red;

    match c {
        Color::Red => println!("coch"),
        Color::Green => println!("gwrydd"),
        Color::Blue => println!("glas"),
    }

    // let size = std::mem::size_of::<Color>();
    // println!("{}", size);
}

fn demo_with_data() {
    println!("\nDemo enums with data");

    // let h = HouseLocation::Number(4);
    // let h = HouseLocation::Name(String::from("cartref"));
    let mut h = HouseLocation::Unknown;

    h = HouseLocation::Number(9);
    match h {
        HouseLocation::Name(s) => println!("You live in a house named {}", s),
        HouseLocation::Number(n) => println!("You live in a house number {}", n),
        HouseLocation::Unknown => println!("Your house location is unknown"),
    }

    let size = std::mem::size_of::<HouseLocation>();
    println!("Btw the size of HouseLocation is {} bytes", size);
}

fn demo_using_option_enum() {
    println!("\nDemo using the Option<T> enum");

    let sec: Option<u32>;

    // sec = sec_of_day(23, 59, 59);
    sec = sec_of_day(2233, 59, 59);

    match sec {
        Some(s) => println!("Second of day: {}", s),
        None => println!("Second of day: no value available"),
    }

    println!("Unwrapped sec: {}", sec.unwrap_or(0));
}

fn sec_of_day(h: u32, m: u32, s: u32) -> Option<u32> {
    if h <= 24 && m <= 59 && s <= 59 {
        let secs = h * 3600 + m * 60 + s;
        return Option::Some(secs);
    } else {
        return Option::None;
    }
}
