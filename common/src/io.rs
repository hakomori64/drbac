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
