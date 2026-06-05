use std::{fmt, string};

fn main() {
    let mut decimal = 10064.43_2f64;

    // only explicit casting allowed
    let int = decimal as u8;
    let cha = int as char;

    print!(
        "Casting decimal -> {} to u8 -> {} to char -> {}",
        decimal, int, cha
    );

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000u32 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    println!("   nan as u8 is : {}", f32::NAN as u8);

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
