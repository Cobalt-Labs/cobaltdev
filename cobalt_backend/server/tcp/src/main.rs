use crate::{tcp_client::tcp_client, tcp_server::tcp_server};

mod tcp_client;
mod tcp_server;

fn main() -> std::io::Result<()> {
    let server = tcp_server()?;
    let client = tcp_client()?;
    println!("{server:?} {client:?}");

    Ok(())
}