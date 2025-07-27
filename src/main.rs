mod dns;

#[allow(unused_imports)]
use dns::Header;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

use crate::dns::{parse_questions, Question};

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
                eprintln!("Error receiving data: {e}");
                break;
            }
        }
    }
}

async fn process(socket: Arc<UdpSocket>, buffer: &[u8], address: SocketAddr) {
    eprintln!("Received {} bytes from {}", buffer.len(), address);

    let header = match Header::from_bytes(&buffer[..12]) {
        Ok(header) => {
            eprintln!("Received header: {header:#?}");
            header
        }
        Err(e) => {
            eprintln!("Error parsing header: {e}");
            return;
        }
    }; 

    let (question, answer_section_offset) = match parse_questions(&buffer[..12]) {
        Ok((question, answer_section_offset)) => {
            eprintln!("Received question: {question:#?}");
            eprintln!("Next offset: {answer_section_offset}");
            (question, answer_section_offset)
        }
        Err(e) => {
            eprintln!("Error parsing question: {e}");
            return;
        }
    };

    let mut response_header = Header::new(header.id);
    response_header.flip_qr();

    let mut response_questions = question;

    // Weird shenanigan to avoid allocating.
    // TODO: Rewrite later with a DNS struct.
    let mut response = response_header.to_bytes().to_vec();
    response.extend_from_slice(&buffer[12..]);
    let mut response_buf = [0u8; 512];
    let header_bytes = response_header.to_bytes();
    let header_len = header_bytes.len();
    response_buf[..header_len].copy_from_slice(&header_bytes);
    let rest_len = buffer.len().saturating_sub(12);
    response_buf[header_len..header_len + rest_len].copy_from_slice(&buffer[12..12 + rest_len]);
    let response_len = header_len + rest_len;

    socket
        .send_to(&response_buf[..response_len], address)
        .await
        .expect("Failed to send response");
    socket
        .send_to(&response, address)
        .await
        .expect("Failed to send response");
}
