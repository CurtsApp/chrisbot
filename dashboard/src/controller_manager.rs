use std::net::UdpSocket;

use gilrs::{Button, Event, Gilrs};
use rcl::network_utils::*;

pub fn watch_controller() {
    let send_socket_result = UdpSocket::bind(DASHBOARD_TX);

    let send_socket = match send_socket_result {
        Ok(send_socket) => send_socket,
        Err(_) => {
            println!("Failed to open landbot TX socket");
            return;
        }
    };

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
        // Examine new events
        while let Some(Event {
            id, event, time, ..
        }) = gilrs.next_event()
        {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
        }
    }
}
