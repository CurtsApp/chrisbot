use rcl::network_utils::*;

pub fn watch_data() -> std::io::Result<()> {
    udp_listen(LANDBOT_RX, rx_data)?;
    Ok(())
}

fn rx_data(data: &mut [u8]) {
    let number_string: String = data.iter().map(|&n| n.to_string()).collect();
    println!("{}", number_string);
}