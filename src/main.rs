#[allow(unused_imports)]
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

#[derive(Debug)]
struct DNSHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DNSHeader {
    pub fn new(header: &[u8]) -> Self {
        assert_eq!(header.len(), 12);
        DNSHeader {
            id: u16::from_be_bytes([header[0], header[1]]),
            flags: u16::from_be_bytes([header[2], header[3]]),
            qdcount: u16::from_be_bytes([header[4], header[5]]),
            ancount: u16::from_be_bytes([header[6], header[7]]),
            nscount: u16::from_be_bytes([header[8], header[9]]),
            arcount: u16::from_be_bytes([header[10], header[11]]),
        }
    }
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = Arc::new(
        UdpSocket::bind("127.0.0.1:2053")
            .await
            .expect("Failed to bind to address"),
    );
    let mut buf = [0u8; 512];

    loop {
        match udp_socket.recv_from(&mut buf).await {
            Ok((size, source)) => {
                let socket = udp_socket.clone();
                tokio::spawn(async move {
                    process(socket, &buf[..size], source).await;
                });
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

async fn process(socket: Arc<UdpSocket>, buffer: &[u8], address: SocketAddr) {
    eprintln!("Received {} bytes from {}", buffer.len(), address);

    assert!(buffer.len() >= 12);

    let header = DNSHeader::new(&buffer[..12]);

    eprintln!("Received header: {:#?}", header);

    let response = [];
    socket
        .send_to(&response, address)
        .await
        .expect("Failed to send response");
}
