fn main() {
    demo_integers();
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
