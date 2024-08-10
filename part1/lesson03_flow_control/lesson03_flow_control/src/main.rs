fn main() {
    // demo_if();
    demo_match();
}

fn demo_if() {
    let age = 58;
    if age > 50 {
        println!("You are old");
    }

    let height = 1.67;
    if height > 1.8 {
        println!("You are tall");
    } else {
        println!("You are not so tall");
    }

    let swans_games = 500;

    if swans_games > 300 {
        println!("You are a loyal fan");
    } else if swans_games > 100 {
        println!("You are a discerning fan");
    } else {
        println!("You are quite a new fan");
    }

    let msg = if age > 50 { "old" } else { "young" };

    println!("You are {}", msg);
}

fn demo_match() {
    let num = 100;

    match num {
        100 => println!("A hundred"), //we call this "arm" in Rust. in other languages
        200 => println!("Two hundred"), // that uses "switch", you'd call it a branch
        _ => println!("Something else"),
    }

    match num {
        25..=50 => println!("25 to 50"),
        51..=100 => println!("51 to 100"),
        _ => println!("Something else"),
    }

    match num {
        25 | 50 | 75 => println!("25, 50, or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Something else"),
    }

    match num {
        x if x < 50 => println!("Less than 50"),
        x if x == 75 => println!("75"),
        _ => println!("something else"),
    }

    let res = match num {
        x if x < 50 => "Less than 50",
        x if x == 75 => "75",
        _ => "something else",
    };

    println!("Result of match expression: {}", res);
}
