// Uncomment this block to pass the first stage
// use std::net::UdpSocket;

use std::net::UdpSocket;


#[allow(dead_code)]
struct DnsHeader{
    id: u16, // Packet Identifier (ID)
    qr: u8, // Query/Response Indicator (QR)
    opcode: u8, // Operation Code (OPCODE)
    aa: u8, //Authoritative Answer (AA)
    tc: u8, //Truncation (TC)
    rd: u8, // Recursion Desired (RD)
    ra: u8, // Recursion Available (RA)
    z: u8, // reserved
    r_code: u8, // Response Code (RCODE)
    qd_count: u16, // Question Count (QDCOUNT)
    an_count: u16, //  Answer Record Count (ANCOUNT)
    ns_count: u16, // Authority Record Count (NSCOUNT)
    ar_count: u16, //  Additional Record Count (ARCOUNT)

}

impl DnsHeader{
    fn pack(&self)-> Vec<u8>{
        let mut buffer = vec![];
        buffer.extend_from_slice(&self.id.to_be_bytes());
        let values = (self.qr as u16) << 15
        |(self.opcode as u16) << 11
            | (self.aa as u16) << 10
            | (self.tc as u16) << 9
            | (self.rd as u16) << 8
            | (self.ra as u16) << 7
            | (self.z as u16) << 4
            | (self.r_code as u16);
        buffer.extend_from_slice(&values.to_be_bytes());
        buffer.extend_from_slice(&self.qd_count.to_be_bytes());
        buffer.extend_from_slice(&self.an_count.to_be_bytes());
        buffer.extend_from_slice(&self.ns_count.to_be_bytes());
        buffer.extend_from_slice(&self.ar_count.to_be_bytes());
        buffer
    }

}

impl Default for DnsHeader{
    fn default() -> Self {
        Self{
            id: 1234,
            qr: 1,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            r_code: 0,
            qd_count: 0,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        }
    }
}

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
                let header_info = DnsHeader::default();
                let response = header_info.pack();
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
