use std::thread;

use crate::event_manager::process_events;

mod data_rx;
mod event_manager;

fn main() -> std::io::Result<()> {
    println!("Hello from bot!");

    let data_rs = thread::spawn(data_rx::watch_data);

    // Infinite loop
    process_events();

    // Wait for all threads to complete
    let _ = data_rs.join();

    Ok(())
}
