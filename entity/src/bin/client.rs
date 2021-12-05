use common::utils::set_current_dir_to_executable_directory;
use entity::client;

fn main() {
    set_current_dir_to_executable_directory().unwrap();
    println!("starting entity client...");
    client::start_client();
}