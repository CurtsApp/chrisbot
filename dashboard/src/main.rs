use std::thread;

mod controller_manager;
mod data_rx;

fn main() -> std::io::Result<()> {
    println!("Dashboard starting");

    // Spawn workers
    let data_thread = thread::spawn(data_rx::watch_data);
    let controller_thread = thread::spawn(controller_manager::watch_controller);

    //Wait for all threads to complete, not expecting them to complete they are infinite loops
    let _ = data_thread.join();
    let _ = controller_thread.join();

    Ok(())
}
