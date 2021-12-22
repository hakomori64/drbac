use anyhow::Result;
use std::process::{Command, Stdio};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("");
        return Ok(());
    }

    let comms = &args[1..];
    println!("{:?}", comms);
    
    let output = if comms.len() <= 1 {
        Command::new(comms[0].clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap()
    } else {
        Command::new(comms[0].clone())
            .args(&comms[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output().unwrap()
    };

    if output.status.success() {
        println!("execution success!!");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("execution failed!!");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}