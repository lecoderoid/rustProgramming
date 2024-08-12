// #![allow(dead_code)]
// mod mytypes;
// use mytypes::Color;

mod mytypes_for_demo_purposes;
use mytypes_for_demo_purposes::{Color, HouseLocation};

fn main() {
    // demo_simple_enums();
    demo_with_data();
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
