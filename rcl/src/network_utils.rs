use std::net::UdpSocket;

use crate::{controller_utils::{Axis, Button}, message_types::{AxisState, ButtonPressedEvent, ButtonState, Message}};

pub const DASHBOARD_RX: &str = "127.0.0.1:34254";
pub const DASHBOARD_TX: &str = "127.0.0.1:34255";
pub const LANDBOT_RX: &str = "127.0.0.1:34256";
pub const LANDBOT_TX: &str = "127.0.0.1:34257";
pub const MAX_UDP_SZ: usize = 512; // 600 byte limit to prevent fragmentation

#[repr(u8)]
pub enum MsgType {
    MsgButtonState,
    MsgAxisState,
}

pub fn udp_listen(address: &str, rx: fn(Message)) -> std::io::Result<()> {
    let socket = UdpSocket::bind(address)?;
    let mut rx_data = [0; MAX_UDP_SZ];
    loop {
        let rx_result = socket.recv_from(&mut rx_data);
        let data_sz = match rx_result {
            Ok(data) => data.0,
            Err(_error) => 0, // On error no data to process
        };

        if data_sz > 0 && data_sz <= MAX_UDP_SZ {
            let rx_data = &mut rx_data[..data_sz];
            let message_result: Result<Message, serde_cbor::Error> =
                serde_cbor::from_slice(rx_data);
            
            match message_result {
                Ok(message) => rx(message),
                Err(_) => (), // On errror no data to process
            }
        }
    }
}

pub struct UDPSender {
    tx_socket: UdpSocket,
    rx_address: String,
}

impl UDPSender {
    pub fn init(tx_address: &str, rx_address: &str) -> Option<UDPSender> {
        match UdpSocket::bind(tx_address) {
            Ok(tx_socket) => Some(UDPSender {
                tx_socket: tx_socket,
                rx_address: rx_address.to_string(),
            }),
            Err(_) => None,
        }
    }

    pub fn send_button_pressed(&self, button: Button) {
        let button_pressed = ButtonPressedEvent { button };
        self.send(&Message::ButtonPressed(button_pressed));
    }

    pub fn send_button_state(&self, button: Button, state: bool) {
        let button_state = ButtonState { button, state };
        self.send(&Message::ButtonState(button_state));
    }

    pub fn send_axis_state(&self, axis: Axis, state: f32) {
        let axis_state = AxisState { axis, state };
        self.send(&Message::AxisState(axis_state));
    }

    fn send(&self, msg: &Message) {
        let cbor_msg = match serde_cbor::to_vec(msg) {
            Ok(msg) => msg,
            Err(_) => {
                println!("Serialize: Failed to send data to {}", self.rx_address);
                return;
            }
        };

        match self.tx_socket.send_to(&cbor_msg, &self.rx_address) {
            Ok(_) => (),
            Err(_) => println!("Send: Failed to send data to {}", self.rx_address),
        }
    }
}
