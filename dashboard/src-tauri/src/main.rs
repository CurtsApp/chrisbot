// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

mod controller_manager;
mod data_rx;

fn main() {
    // Start background processing
    println!("Dashboard starting");

    // Spawn workers
    let _ = thread::spawn(data_rx::watch_data);
    //controller_manager::watch_controller();
    let _ = thread::spawn(controller_manager::watch_controller);

    dashboard_lib::run();

    println!("Dashboard ending");
}
