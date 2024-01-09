// Uncomment this block to pass the first stage
// use std::net::UdpSocket;

mod message;
use std::net::UdpSocket;
use message::{DnsHeader, DnsQuestion};
use crate::message::DnsAnswer;


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                //let response = [];
                let mut header_info = DnsHeader::default();
                header_info.qd_count += 1;
                header_info.an_count += 1;
                let mut  response = header_info.pack();
                let q = DnsQuestion::default();
                println!("Question type {:?}", q);
                let question = q.to_bytes();
                //let question = DnsQuestion::default().to_bytes();
                let answer = DnsAnswer::get_answer();

                response.extend_from_slice(&question);
                response.extend_from_slice(&answer);
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
