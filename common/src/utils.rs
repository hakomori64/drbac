use anyhow::{Result};
use std::env;

pub fn set_current_dir_to_executable_directory() -> Result<()> {
    let mut path = env::current_exe()?;
    path.pop();
    println!("setting current directory to {:?}", path.display());
    env::set_current_dir(path)?;

    Ok(())
}