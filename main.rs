use std::{fmt, string};

// What has Go done to me
// I can't tell if I hate or love these enums
enum Event {
    PageLoad,
    PageUnload,

    Click(char),
    Scroll(i128),

    Position { x: i128, y: i128 },
}

fn inspect(e: Event) {
    match e {
        Event::PageLoad => println!("PageLoad"),
        Event::PageUnload => println!("PageUnLoad"),
        Event::Click(c) => println!("Clicked {}", c),
        Event::Scroll(f) => println!("Scrolled {} pixels", f),
        Event::Position { x, y } => println!("At position: ({}, {})", x, y),
    }
}

type String = Event; // Hello type aliases

fn main() {
    let load = Event::PageLoad;
    let position = String::Position { x: 2, y: (3) };

    inspect(load);
    inspect(position);
}
