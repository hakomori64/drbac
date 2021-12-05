use anyhow::Result;
use std::path::PathBuf;
use serde::Serialize;
use std::io;
use serde::de::DeserializeOwned;
use std::io::BufReader;
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

pub fn write_json_to_file<T: Serialize>(filename: &PathBuf, data: T) -> Result<()> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    let text = serde_json::to_string(&data)?;
    write!(&file, "{}", text)?;

    Ok(())
}
pub fn read_json_from_file<T: DeserializeOwned>(filename: &PathBuf) -> Result<T> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)?;
    
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}