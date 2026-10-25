use std::io::{Read, Write};
use std::net::TcpListener;

pub fn tcp_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.0.1:8080")?;
    println!("Listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                println!("client connected!");
                socket.write_all(b"Hello")?;
                let mut buffer = [0u8; 1024];

                loop {
                    let bytes = socket.read(&mut buffer)?;
                    if bytes == 0 {
                        println!("client disconnected!");
                        break;
                    }
                    let message = String::from_utf8_lossy(&buffer[..bytes]);

                    println!("recieved {message}");

                    socket.write_all(message.as_bytes())?;
                }
            }
            Err(e) => eprintln!("connected failed {e}"),
        }
    }
    Ok(())
}
