#![allow(dead_code)]
mod mytypes;
use mytypes::Color;

// mod mytypes_for_demo_purposes;
// use mytypes_for_demo_purposes::{Color, HouseLocation};

fn main() {
    demo_simple_enums();
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
fn main2() {
    demo_simple_enums();
}
