use std::collections::HashMap;

fn main() {
    // demo_arrays();
    // demo_array_techniques();
    // demo_tuples();
    // demo_vectors();
    demo_maps();
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

fn demo_vectors() {
    println!("\nUsing Vectors");

    // you can create a vector object using either of the following syntaxes
    let mut _v1: Vec<i32> = Vec::new();
    let mut _v2 = Vec::<i32>::new();

    //you can creat a vector object and initialize it via the vec! macro
    let mut v3 = vec![100, 101, 102];

    let item = v3[0];
    println!("Value: {}", item);

    //index into a vector safely, returns an Option<T>
    let opt = v3.get(0);
    match opt {
        Some(v) => println!("Value: {}", v),
        None => println!("No value"),
    }

    //add and remove items in a vector
    v3.push(103);
    v3.push(104);
    v3.push(105);
    v3.pop();
    v3.insert(0, 99);
    v3[1] = 43;

    // iterate over the items
    for item in &v3 {
        println!("{} ", item);
    }

    //display the vector all at once
    println!("\nv3 is {:?}", v3);
}

fn demo_maps() {
    println!("\nUsing Maps");

    //you can create a map object using either of the ff syntaxes
    let mut m: HashMap<String, i32> = HashMap::new();
    let mut _m2 = HashMap::<String, i32>::new();

    //insert items
    m.insert(String::from("UK"), 44);
    m.insert(String::from("NO"), 47);
    m.insert(String::from("SG"), 65);

    //insert item only if key is missing
    m.entry(String::from("SA")).or_insert(27);

    //look-up key (will panic ifkeyis missing)
    let val = m["UK"];
    println!("Value for key 'UK': {}", val);

    //look up a key safely, returns an Option<V>
    let opt = m.get("UK");

    match opt {
        Some(v) => println!("Value for key 'UK': {}", v),
        None => println!("No value"),
    }

    println!("Entries in m:");

    for entry in &m {
        println!(" {:?}", entry);
    }

    //display a map all at once via the debug formatter
    println!("m is {:?}", m);
}
