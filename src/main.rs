// Uncomment this block to pass the first stage
// use std::net::UdpSocket;

use std::net::UdpSocket;


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
                let header_info = write_header_section();
                let response = unsafe { any_as_u8_slice(&header_info) };
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


fn write_header_section()-> DnsHeader{

    DnsHeader{
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

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        core::mem::size_of::<T>(),
    )
}