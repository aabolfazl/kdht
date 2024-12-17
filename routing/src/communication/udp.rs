use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::time::Duration;

const MAX_PACKET_SIZE: usize = 65507;
const RECEIVE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct Message {
    pub data: Vec<u8>,
    pub sender: Sender,
}

#[derive(Debug, Clone)]
pub struct Sender {
    pub address: SocketAddr,
    pub port: u16,
}

pub trait Connection {
    fn receive(&self) -> Pin<Box<dyn Future<Output=Result<Message, std::io::Error>> + Send + '_>>;
    fn send(
        &self,
        message: &Message,
    ) -> Pin<Box<dyn Future<Output=Result<usize, std::io::Error>> + Send + '_>>;
}

pub struct UdpConnection {
    socket: Arc<UdpSocket>,
    address: SocketAddr,
    buffer_size: usize,
    retry_count: u32,
}

impl UdpConnection {
    pub async fn new(
        address: SocketAddr,
        buffer_size: Option<usize>,
        retry_count: Option<u32>,
    ) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(address).await?;

        Ok(Self {
            socket: Arc::new(socket),
            address,
            buffer_size: buffer_size.unwrap_or(MAX_PACKET_SIZE),
            retry_count: retry_count.unwrap_or(3),
        })
    }
}

impl Connection for UdpConnection {
    fn receive(
        &self,
    ) -> Pin<Box<dyn Future<Output=Result<Message, std::io::Error>> + Send + '_>> {
        let socket = Arc::clone(&self.socket);
        let mut buffer = vec![0; self.buffer_size];

        Box::pin(async move {
            let (size, src_addr) = socket.recv_from(&mut buffer).await?;
            Ok(Message {
                data: buffer[..size].to_vec(),
                sender: Sender {
                    address: src_addr,
                    port: src_addr.port(),
                },
            })
        })
    }

    fn send(
        &self,
        message: &Message,
    ) -> Pin<Box<dyn Future<Output=Result<usize, std::io::Error>> + Send + '_>> {
        let socket = Arc::clone(&self.socket);
        let data = message.data.clone();
        let addr = message.sender.address;
        let retry_count = self.retry_count;

        Box::pin(async move {
            for attempt in 1..=retry_count {
                match socket.send_to(&data, &addr).await {
                    Ok(size) => return Ok(size),
                    Err(e) if attempt == retry_count => return Err(e),
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(100 * attempt as u64)).await;
                        continue;
                    }
                }
            }
            Ok(0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_udp_connection() {
        let server_addr = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), 12345);
        let server = UdpConnection::new(server_addr, None, None).await.unwrap();

        let server_handle = tokio::spawn(async move {
            let received = server.receive().await.unwrap();
            assert_eq!(received.data, b"ping");
            received
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let client_addr = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), 0);
        let client = UdpConnection::new(client_addr, None, None).await.unwrap();

        let message = Message {
            data: b"ping".to_vec(),
            sender: Sender {
                address: server_addr,
                port: server_addr.port(),
            },
        };

        client.send(&message).await.unwrap();
        let data = server_handle.await.unwrap();
        assert_eq!(data.data, b"ping");
        assert_eq!(data.sender.address, client_addr);
        assert_eq!(data.sender.port, client_addr.port());
    }
}
