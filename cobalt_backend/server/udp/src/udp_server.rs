use std::net::UdpSocket;

pub fn udp_server() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    println!("UDP server listening on 127.0.0.1:8080");

    let mut buffer = [0u8; 1024];

    loop {
        let (size, client_addr) = socket.recv_from(&mut buffer)?;

        let message = String::from_utf8_lossy(&buffer[..size]);

        println!("Received from {}: {}", client_addr, message);

        let response = format!("Server received: {}", message);

        socket.send_to(response.as_bytes(), client_addr)?;
    }
}
