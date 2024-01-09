#[allow(dead_code)]
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
    an_count: u16, //  Answer Record Count (ANCOUNT)
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



pub struct DnsQuestion{
    domain_name: String,
    question_type: u16,
    question_class: u16,
}

impl Default for  DnsQuestion{
    fn default() -> Self {
        Self{
            domain_name: "".to_string(),
            question_type: 0,
            question_class: 0,
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

        let domain_name = domain_name.split_once('.').expect("Could not split the domain name");
        println!("{:?}", domain_name);
        let label1_len = domain_name.0.len();
        let label2_len = domain_name.1.len();
        //encoded_value.extend_from_slice(format!("{:x}", label1_len).as_bytes());
        encoded_value.push(label1_len as u8);
        encoded_value.extend_from_slice(domain_name.0.as_bytes());
        //encoded_value.extend_from_slice(format!("{:x}", label2_len).as_bytes());
        encoded_value.push(label2_len as u8);
        encoded_value.extend_from_slice(domain_name.1.as_bytes());
        encoded_value.push(0);
        encoded_value.extend_from_slice(&self.question_type.to_be_bytes());
        encoded_value.extend_from_slice(&self.question_class.to_be_bytes());
        encoded_value
    }


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