mod mytypes;
use mytypes::Color;

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
}
