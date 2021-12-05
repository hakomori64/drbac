use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    let mut child = Command::new("bash")
        .arg("-m")
        .spawn()
        .expect("bash failed to start");
    
    child.wait().expect("child process finished unexpectedly");

    Ok(())
}