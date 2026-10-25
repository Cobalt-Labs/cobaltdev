use std::io::{Read, Write};
use std::net::TcpStream;

pub fn tcp_client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.0.1:8080")?;

    let mut buffer = [0u8; 1024];
    let bytes = stream.read(&mut buffer)?;

    println!("server says {}", String::from_utf8_lossy(&buffer[..bytes]));

    let _ = stream.write_all(b"my name is ibrahim");

    let bytes = stream.read(&mut buffer)?;

    println!("echo {}", String::from_utf8_lossy(&buffer[..bytes]));

    Ok(())
}
