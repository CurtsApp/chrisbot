use std::{
    net::UdpSocket,
    time::{Duration, Instant},
};

use rcl::network_utils::*;

// Process events in infinite loop
pub fn process_events() {
    let mut last_update = Instant::now();

    let send_socket_result = UdpSocket::bind(LANDBOT_TX);

    let send_socket = match send_socket_result {
        Ok(send_socket) => send_socket,
        Err(_) => {
            println!("Failed to open landbot TX socket");
            return;
        }
    };

    loop {
        let now = Instant::now();

        // One second periodic processing
        if now.duration_since(last_update) > Duration::from_secs(1) {
            one_sec_proc(&send_socket);
            last_update = now;
        }
    }
}

fn one_sec_proc(tx: &UdpSocket) {
    static mut COUNTER: u16 = 0;
    // Send test packets here
    unsafe {
        match tx.send_to(&COUNTER.to_be_bytes(), DASHBOARD_RX) {
            Ok(_) => (),
            Err(_) => println!("Failed to send data to dashboard"),
        }

        COUNTER += 1;
    }
}
