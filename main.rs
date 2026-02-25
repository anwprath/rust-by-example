/// This main function prints `Hello world!`
fn main() {
    let (a, b): (i128, i128) = (128222, 16);

    let mut line: String = format!("{}, {}", a, b);
    print!("{}", line);

    line = format!("{1}, {0}", a, b);
    println!("{}", line);

    // args within the statement
    println!("{arg1}, {arg2}", arg1=a, arg2=b);

    println!("Base 10:               {},   {}",   a, b);
    println!("Base 2 (binary):       {:b}, {:b}", a, b);
    println!("Base 8 (octal):        {:o}, {:o}", a, b);
    println!("Base 16 (hexadecimal): {:x}, {:x}", a, b);

    // pad by len(n) - 5 whitespaces to the left / right-justify
    println!("{n:>10}", n=a);

    // pad by len(n) - 5 Xs to the left / right-justify
    println!("{n:X>10}", n=a);

    #[derive(Debug)] 
    struct Structure(i32);

    println!("This struct `{:?}`,  will print... if you annotate it with the Debug trait", Structure(3));
    println!("This struct `{:#?}`,  will pretty print... if you annotate it with the Debug trait", Structure(3));


    // capture variables from scope
    let width: usize = 17;
    println!("{a:>width$}");
}
