#[allow(unused_imports)]
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;
// use std::net::UdpSocket;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
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

async fn process(socket: Arc<tokio::net::UdpSocket>, buffer: &[u8], address: SocketAddr) {
    eprintln!("Received {} bytes from {}", buffer.len(), address);

    let response = [];
    socket
        .send_to(&response, address)
        .await
        .expect("Failed to send response");
}
