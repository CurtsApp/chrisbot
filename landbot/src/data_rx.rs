use rcl::{message_types::Message, network_utils::*};

pub fn watch_data() -> std::io::Result<()> {
    udp_listen(LANDBOT_RX, rx_data)?;
    Ok(())
}

fn rx_data(message: Message) {
    match message {
        Message::AxisState(axisState) => {
            println!("{:?}", axisState);
        }
        Message::ButtonPressed(buttonPressed) => {
            println!("{:?}", buttonPressed);
        }
        Message::ButtonState(buttonState) => {
            println!("{:?}", buttonState);
        }
    }
}
