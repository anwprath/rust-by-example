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
    let logical = true;
    let logical_annotated: bool = false;

    let default_float = 3.0; //f64
    let float_annotated: f32 = 1.0;

    let mut big_int: i128 = 1000000000000000; // unannotated assigns i32
    // let small_int = 10000000000000; // compiler error

    let mut arr: [i32; 5] = [1,2,3,4,5]; // basic array type
    let tuple: (i64,bool, i64) = (1,true,3);
}
