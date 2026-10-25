use crate::{udp_client::udp_client, udp_server::udp_server};

mod udp_server;
mod udp_client;

fn main() -> std::io::Result<()> {
    let server = udp_server()?;
    let client = udp_client()?;
    println!("{server:?} {client:?}");

    Ok(())
}
