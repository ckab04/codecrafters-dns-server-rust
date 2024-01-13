use std::net::Ipv4Addr;
use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
#[repr(C, packed)]
pub struct DnsHeader{
    id: u16, // Packet Identifier (ID)
    qr: u8, // Query/Response Indicator (QR)
    opcode: u8, // Operation Code (OPCODE)
    aa: u8, //Authoritative Answer (AA)
    tc: u8, //Truncation (TC)
    rd: u8, // Recursion Desired (RD)
    ra: u8, // Recursion Available (RA)
    z: u8, // reserved
    r_code: u8, // Response Code (RCODE)
    pub qd_count: u16, // Question Count (QDCOUNT)
    pub an_count: u16, //  Answer Record Count (ANCOUNT)
    ns_count: u16, // Authority Record Count (NSCOUNT)
    ar_count: u16, //  Additional Record Count (ARCOUNT)

}

impl DnsHeader{
    pub(crate) fn pack(&self) -> Vec<u8>{
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

    pub(crate) unsafe fn parse_header(packet_data: &[u8]) -> DnsHeader{

        let mut buffer: DnsHeader = std::mem::zeroed();
        let mut header_bytes = std::slice::from_raw_parts_mut(&mut buffer as *mut _ as *mut u8, std::mem::size_of::<DnsHeader>());
        println!("Size of Dns Header = {}", std::mem::size_of::<DnsHeader>());
        header_bytes.copy_from_slice(&packet_data[0..12]);
       /* header_bytes = &mut *Bytes::copy_from_slice(&packet_data[0..12]);
        let bytes_reader = header_bytes.reader();
        buffer.id = Bytes::rea*/

        buffer.id = BigEndian::read_u16(&header_bytes[0..2]);
        let flags = BigEndian::read_u16(&header_bytes[2..4]);
        buffer.qd_count = BigEndian::read_u16(&header_bytes[4..6]);
        buffer.an_count = BigEndian::read_u16(&header_bytes[6..8]);
        buffer.ns_count = BigEndian::read_u16(&header_bytes[8..10]);
        buffer.ar_count = BigEndian::read_u16(&header_bytes[10..12]);
        let flag1 = (flags >> 8) as u8 & 0x00ff;
        let flag2 = ((flags & 0xff00) >> 8) as u8;
        buffer.qr= (flag1 >> 7) & 0x01;
        buffer.opcode = (flag1 >> 3)  & 0x0f;
        buffer.aa = (flag1 >> 2) & 0x01;
        buffer.tc = (flag1 >> 1) & 0x01;
        buffer.rd = flag1 & 0x01;

        buffer.ra = (flag2 >> 7) & 0x01;
        buffer.z = (flag2 >> 4) & 0x07;
        buffer.r_code = (flag2) & 0x0f;
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



#[derive(Debug)]
pub struct DnsQuestion{
    domain_name: String,
    question_type: u16,
    question_class: u16,
}

impl Default for  DnsQuestion{
    fn default() -> Self {
        Self{
            domain_name: "codecrafters.io".to_string(),
            question_type: 1,
            question_class: 1,
        }
    }
}

impl DnsQuestion{

    pub(crate) fn to_bytes(&self) -> Vec<u8>{

        let mut question = Self::default();
        question.domain_name = String::from("codecrafters.io");
        question.question_type = 1;
        question.question_class = 1;
        self.conversion(question.domain_name)
    }

    fn conversion(&self, domain_name: String) -> Vec<u8>{
        let mut encoded_value: Vec<u8> = Vec::new();
        encoded_value = encoded_label(&self.domain_name);
        encoded_value.extend_from_slice(&self.question_type.to_be_bytes());
        encoded_value.extend_from_slice(&self.question_class.to_be_bytes());
        encoded_value
    }

}

fn encoded_label(domain_name: &str)-> Vec<u8>{
    let mut encoded_value: Vec<u8> = Vec::new();

    domain_name.split('.').for_each(|label|{
        encoded_value.push(label.len() as u8);
        encoded_value.extend_from_slice(label.as_bytes());
    }
    );
    encoded_value.push(0);
    encoded_value
}


#[allow(dead_code)]
#[repr(C)]
enum TYPE{
    A = 1, // a host address
    NS=2, // an authoritative name server
    MD =3, // , //a mail destination (Obsolete - use MX)
    MF = 4, // a mail forwarder (Obsolete - use MX)
    CNAME =5, // the canonical name for an alias
    SOA  = 6, // marks the start of a zone of authority
    MB  = 7, // a mailbox domain name (EXPERIMENTAL)
    MG = 8, // a mail group member (EXPERIMENTAL)
    MR  = 9 , //a mail rename domain name (EXPERIMENTAL)
    NULL= 10, // a null RR (EXPERIMENTAL)
    WKS=  11, // a well known service description
    PTR = 12, // a domain name pointer
    HINFO =13, // host information
    MINFO =14, // mailbox or mail list information
    MX= 15 , //mail exchange
    TXT= 16 , //text strings
}


#[allow(dead_code)]
#[repr(C)]
enum CLASS{
    IN  =1,// the Internet
    CS     = 2,// the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CH      =3,// the CHAOS class
    HS    = 4,// Hesiod [Dyer 87]
}

#[derive(Debug)]
pub struct DnsAnswer{
    name: String,
    type_answer: u16,
    class: u16,
    ttl: u32,
    length: u16,
    data: String,
}

impl DnsAnswer{

    pub fn get_answer() -> Vec<u8>{

        let mut encoded_anwer: Vec<u8> = Vec::new();

        let answer = DnsAnswer{
            name: String::from("codecrafters.io"),
            type_answer: 1,
            class: 1,
            ttl: 60,
            length: 4,
            data: String::from("8.8.8.8"),
        };
        println!("Answer  type {:?}", answer);
        let encoded_domain_name = encoded_label(&answer.name);
        encoded_anwer.extend_from_slice(&encoded_domain_name);
        encoded_anwer.extend_from_slice(&answer.type_answer.to_be_bytes());
        encoded_anwer.extend_from_slice(&answer.class.to_be_bytes());
        encoded_anwer.extend_from_slice(&answer.ttl.to_be_bytes());
        encoded_anwer.extend_from_slice(&answer.length.to_be_bytes());

        let ipv4 = answer.data.parse::<Ipv4Addr>().expect("Could not parse the ipv4 Address").octets();
        encoded_anwer.extend_from_slice(&ipv4);
        encoded_anwer

    }

}

