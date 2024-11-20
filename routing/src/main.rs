mod communication;

use crate::communication::udp::UdpConnection;
use crate::communication::{Connection, Message};
use communication::udp;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::net::SocketAddr;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let address: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let connection: UdpConnection = UdpConnection::new(address).await?;

    loop {
        connection.receive(|message: Message| {
            info!(
                    "Received message: size: {} protocol: {} message address: {}",
                    message.data.len(),
                    message.protocol,
                    message.sender.address
                );
        }).await?;
    }
}
