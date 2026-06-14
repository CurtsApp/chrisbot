use std::net::UdpSocket;

pub const DASHBOARD_RX: &str = "127.0.0.1:34254";
pub const DASHBOARD_TX: &str = "127.0.0.1:34255";
pub const LANDBOT_RX: &str = "127.0.0.1:34256";
pub const LANDBOT_TX: &str = "127.0.0.1:34257";
pub const MAX_UDP_SZ: usize = 512; // 600 byte limit to prevent fragmentation

pub fn udp_listen(address: &str, rx: fn(&mut [u8])) -> std::io::Result<()> {
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
            rx(rx_data);
        }
    }
}
