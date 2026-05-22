use std::{fmt, string};

fn main() {
    let mut binding: u32;
    let mut x = 5;
    println!("x: {}", x);
    {
        x = 2;
        binding = x * x;
    }
    println!("x: {}", x);
    println!("binding: {}", binding);

    // this is so dumb
    let binding = 128i128;
    println!("shadowed binding: {}", binding);

    let mut _mutable_integer = 7i32;
    println!("_mutable_integer: {}", _mutable_integer);

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer + 5;
        println!("_mutable_integer: {}", _mutable_integer);

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50; // not allowed obviously

        // `_mutable_integer` goes out of scope
    }
    println!("_mutable_integer: {}", _mutable_integer);

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
