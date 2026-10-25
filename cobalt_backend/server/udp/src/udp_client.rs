use std::net::UdpSocket;

pub fn udp_client() -> std::io::Result<()>{
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    let server_addr = "127.0.0.1:8080";

    let message = "Hello UDP server!";

    socket
        .send_to(message.as_bytes(), server_addr)?;

    println!("Sent: {}", message);

    let mut buffer = [0u8; 1024];

    let (size, _) = socket
        .recv_from(&mut buffer)?;

    let response = String::from_utf8_lossy(&buffer[..size]);

    println!("Response: {}", response);

    Ok(())
}