use std::collections::HashMap;

fn main() {
    // demo_arrays();
    demo_array_techniques();
}

fn demo_arrays() {
    println!("\nUsing arrays");

    let a1 = [100, 101, 102];

    println!("a1 length is {}, first element is {}", a1.len(), a1[0]);

    // you can also create an array - you can change items, but you can't change the size
    let mut a2 = [100, 101, 102];
    a2[0] = 999;
    println!("a2 length is {}, first element is {}", a2.len(), a2[0]);

    // you can iterate over the elements in an array
    println!("Elements in a2");
    for elem in a2 {
        print!("{} ", elem);
    }

    println!("");
}

fn demo_array_techniques() {
    println!("\nArray techniques");

    //you can specify type infor and size
    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("a1 is {:?}", a1);

    //you can fill an array with [filler; size] syntax
    let mut a2 = [99; 5];

    a2[0] = 43;
    a2[4] = 42;
    println!("a2 is {:?}", a2);
}
