use anyhow::{Result};
use std::env;
use std::time::Duration;

pub fn set_current_dir_to_executable_directory() -> Result<()> {
    let mut path = env::current_exe()?;
    path.pop();
    println!("setting current directory to {:?}", path.display());
    env::set_current_dir(path)?;

    Ok(())
}

pub fn print_time(duration: Duration) -> () {
    println!("{}.{:03}秒経過しました", duration.as_secs(), duration.subsec_nanos() / 1_000_000);
}