use std::collections::HashMap;

fn main() {
    // demo_arrays();
    // demo_array_techniques();
    demo_tuples();
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

fn demo_tuples() {
    // a tuple is a fixed-sized heterogeneous collection
    let t1 = (2, "h1", 3.42);
    println!("t1 elements are {}, {}, {}", t1.0, t1.1, t1.2);

    // you can also create a mut tuple ( you have to be consistent with element types)
    let mut t2 = (32, "hello", 23.3);
    t2.0 = 43;
    println!("t2 elements are {}, {}, {}", t2.0, t2.1, t2.2);

    // you can create an empty tuple (handy for functions that returns nothing)
    let t3 = ();
    println!("t3 is {:?}", t3);

    //you can specify the type info
    let t4: (i32, bool, bool, f64);
    t4 = (43, true, false, 3.14);

    println!(
        "t4 is {:?}, elements are {}, {}, {}, {}",
        t4, t4.0, t4.1, t4.2, t4.3
    );
}
