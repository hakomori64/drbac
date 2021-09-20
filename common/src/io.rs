use std::io;
use std::io::prelude::*;


pub fn write(message: &str) -> () {
    print!("{}", message);
    io::stdout().flush().expect("cannot write message");
}

pub fn read_line<U: std::str::FromStr>() -> U {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read input.");
        
        let input = match input.trim().parse::<U>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => continue,
        };
        return input;
    }
}

pub fn read_until<U, F>(
    before_read_message: &str,
    invalid_message: &str,
    condition: F
) -> U
where
    U: std::str::FromStr,
    F: Fn(&U) -> bool
{
    write(before_read_message);
    let mut val: U = read_line();
    while !condition(&val) {
        println!("{}", invalid_message);
        write(before_read_message);
        val = read_line();
    }
    val
}