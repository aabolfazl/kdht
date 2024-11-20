use crate::communication::{Connection, Message, ProtocolType, Sender};
use log::info;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct UdpConnection {
    socket: UdpSocket,
    address: SocketAddr,
}

impl UdpConnection {
    pub async fn new(address: SocketAddr) -> Result<Self, std::io::Error> {
        let socket: UdpSocket = UdpSocket::bind(address).await?;
        info!("Listening on: {}", address);
        Ok(Self { socket, address })
    }
}

impl Connection for UdpConnection {
    async fn receive(&self, process_message: impl Fn(Message) + Copy) -> Result<(), std::io::Error> {
        let mut buffer = [0; 1024];
        loop {
            let (size, _) = self.socket.recv_from(&mut buffer).await?;
            info!("Received {} bytes", size);
            let message = Message {
                data: buffer[..size].to_vec(),
                protocol: ProtocolType::Udp,
                sender: Sender {
                    address: self.address,
                    port: self.address.port(),
                },
            };
            process_message(message);
        }
    }

    async fn send(&self, message: &Message) -> Result<usize, std::io::Error> {
        let res = self.socket.send_to(&message.data, &message.sender.address).await?;

        info!("Message sent to: {}", message.sender.address);
        Ok(res)
    }
}