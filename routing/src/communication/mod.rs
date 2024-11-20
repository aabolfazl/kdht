use std::fmt;
use std::net::SocketAddr;

pub(crate) mod udp;

#[derive(Debug)]
pub enum ProtocolType { Udp }

pub trait Connection {
    async fn receive(&self, process_message: impl Fn(Message) + Copy) -> Result<(), std::io::Error>;
    async fn send(&self, message: &Message) -> Result<usize, std::io::Error>;
}

#[derive(Debug)]
pub struct Sender {
    pub address: SocketAddr,
    pub port: u16,
}

#[derive(Debug)]
pub struct Message {
    pub data: Vec<u8>,
    pub protocol: ProtocolType,
    pub sender: Sender,
}

impl fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolType::Udp => write!(f, "UDP"),
        }
    }
}