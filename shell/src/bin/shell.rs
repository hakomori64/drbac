use anyhow::Result;
use std::process::Command;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("");
        return Ok(());
    }

    let comms = &args[1..];
    println!("{:?}", comms);
    
    let mut child = if comms.len() <= 1 {
        Command::new(comms[0].clone())
            .spawn()
            .unwrap()
    } else {
        Command::new(comms[0].clone())
            .args(&comms[1..])
            .spawn().unwrap()
    };
    child.wait().unwrap();
    Ok(())
}