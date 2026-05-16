use std::{fmt, string};

#[derive(PartialEq, Debug)]
struct Unit; // Go equivalent of type Unit struct{}

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}



fn main() {
    let person = Person {name: String::from("Dumbass"), age:2};
    println!("{:#?}", person);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 12.3, ..another_point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let unit1 = Unit{};
    let unit2 = Unit{};

    assert_eq!(unit1, unit2); // Point to different objects though
    println!("{:p}, {:p}", &unit1, &unit2);

}
