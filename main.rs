use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure({}: i32)", self.0)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for (index, item) in self.0.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", index, item)?;
        }

        write!(f, "]")
    }
}

fn _print_stuff() {
    let (a, b): (i128, i128) = (128222, 16);

    let mut line: String = format!("{}, {}", a, b);
    print!("{}", line);

    line = format!("{1}, {0}", a, b);
    println!("{}", line);

    // args within the statement
    println!("{arg1}, {arg2}", arg1 = a, arg2 = b);

    println!("Base 10:               {},   {}", a, b);
    println!("Base 2 (binary):       {:b}, {:b}", a, b);
    println!("Base 8 (octal):        {:o}, {:o}", a, b);
    println!("Base 16 (hexadecimal): {:x}, {:x}", a, b);

    // pad by len(n) - 5 whitespaces to the left / right-justify
    println!("{n:>10}", n = a);

    // pad by len(n) - 5 Xs to the left / right-justify
    println!("{n:X>10}", n = a);

    println!(
        "This struct `{:?}`,  will print... if you annotate it with the Debug trait",
        Structure(3)
    );
    println!(
        "This struct `{:#?}`,  will pretty print... if you annotate it with the Debug trait",
        Structure(3)
    );
    println!(
        "This struct `{}`,  will use fmt::Display... if you implement fmt::Display for it",
        Structure(3)
    );

    println!("List: {:?}", List(vec![0, 1, 2, 3]));
    println!("List: {}", List(vec![0, 1, 2, 3]));

    // capture variables from scope
    let width: usize = 17;
    println!("{a:>width$}");
}

/// This main function prints `Hello world!`
fn main() {
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    let a = 10;
    let b = 20;
    println!("1 - 2 = {}", 1u32 +(a+b));
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
