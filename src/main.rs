mod dns;

#[allow(unused_imports)]
use dns::DNSHeader;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

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

    let header = DNSHeader::from_bytes(&buffer[..12]);

    eprintln!("Received header: {:#?}", header);

    // let response_header = DNSHeader {
    //     id: header.id,
    //     flags: 1 << 15,
    //     qdcount: 0,
    //     ancount: 0,
    //     nscount: 0,
    //     arcount: 0,
    // };
    let mut response_header = DNSHeader::new(header.id);
    response_header.flip_qr();

    let response = response_header.to_bytes();

    socket
        .send_to(&response, address)
        .await
        .expect("Failed to send response");
}
