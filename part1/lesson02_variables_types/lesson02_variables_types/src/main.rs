fn main() {
    // demo_integers();
    // demo_floats();
    demo_other_simple_types();
}

fn demo_integers() {
    // rust has signed int types i8, i16, i32, i64, i128
    let a1: i32 = -12345; //decimal
    let a2: i32 = 0xFFFF; //hex
    let a3: i32 = 0o177; //octal
    let a4: i32 = 0b1110; //binar

    // rust has unsigned int types u8, u16, u32, u64, u128
    let b: u32 = 1234;

    //rust has architecture-specific int type isize, usize
    let c: isize = 24680;

    println!("\nNumbers are {} {} {} {} {} {}", a1, a2, a3, a4, b, c);
    println!(
        "Numbers in reverse order are {5} {4} {3} {2} {1} {0}",
        a1, a2, a3, a4, b, c
    );
    println!(
        "isize is {} bytes on my machine",
        std::mem::size_of::<isize>()
    );
}

fn demo_floats() {
    //rust has single-precicision and double-precicision floats
    let f1: f32 = 1.23456;
    let f2: f64 = 9.87654;

    println!("\nFloats are {} {}", f1, f2);
    println!("\nFloats to 2dp are {:.2} {:.2}", f1, f2);
    println!(
        "\nfloats field width 10 l-aligned filled with space are ***{:<10.2}*** and ***{:<10.2}***",
        f1, f2
    );
    println!(
        "\nfloats field width 10 R-aligned filled with space are ***{:>10.2}*** and ***{:>10.2}***",
        f1, f2
    );
    println!(
        "\nFloats field width 10 L-aligned filled with tilde are ***{:~<10.2}*** and ***{:~<10.2}***",
        f1, f2
    );
    println!(
        "\nFloats field width 10 R-aligned filled with tilde are ***{:~>10.2}*** and ***{:~>10.2}***",
        f1, f2
    );

    // scientific notation with floats
    let f3: f32 = 1.60217663e-16;
    let f4: f64 = 2.99792458e8;

    println!("\nElectron charge {0}, {0:e}, {0:.4e}", f3);
    println!("\nSpeed of light {0}, {0:e}, {0:.4e}", f4);
}

fn demo_other_simple_types() {
    let is_human: bool = true;
    let is_flat_earther = false;

    println!(
        "\nIs human? {}, is flat_earther? {}",
        is_human, is_flat_earther
    );

    let res1: bool = is_human && is_flat_earther;
    let res2: bool = is_human || is_flat_earther;
    let res3: bool = !(is_human || is_flat_earther);
    println!("res1: {}, res2: {}, res3 {}", res1, res2, res3);

    let first_letter_of_the_alphabet = 'a';
    let my_emoji = '🥲';
    println!(
        "\nFirst letter of the English Alphabet is '{}', and my fave emoji is '{}'",
        first_letter_of_the_alphabet, my_emoji
    );
}
