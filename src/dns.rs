#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Header {
    pub fn from_bytes(header: &[u8]) -> Result<Self, String> {
        assert_eq!(header.len(), 12);
        if header.len() != 12 {
            return Err(format!("Header size is {}", header.len()));
        }

        Ok(Header {
            id: u16::from_be_bytes([header[0], header[1]]),
            flags: u16::from_be_bytes([header[2], header[3]]),
            qdcount: u16::from_be_bytes([header[4], header[5]]),
            ancount: u16::from_be_bytes([header[6], header[7]]),
            nscount: u16::from_be_bytes([header[8], header[9]]),
            arcount: u16::from_be_bytes([header[10], header[11]]),
        })
    }

    pub fn new(id: u16) -> Self {
        Header {
            id,
            flags: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let id = self.id.to_be_bytes();
        let flags = self.flags.to_be_bytes();
        let qdcount = self.qdcount.to_be_bytes();
        let ancount = self.ancount.to_be_bytes();
        let nscount = self.nscount.to_be_bytes();
        let arcount = self.arcount.to_be_bytes();

        [
            id[0], id[1], flags[0], flags[1], qdcount[0], qdcount[1], ancount[0], ancount[1],
            nscount[0], nscount[1], arcount[0], arcount[1],
        ]
    }

    pub fn flip_qr(&mut self) {
        self.flags ^= 1 << 15
    }
}

#[derive(Debug)]
pub struct Question<'a> {
    qname: &'a [u8],
    qtype: QType,
    qclass: QClass,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum QType {
    A,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
}

impl From<u16> for QType {
    fn from(value: u16) -> Self {
        match value {
            1 => QType::A,
            2 => QType::NS,
            3 => QType::MD,
            4 => QType::MF,
            5 => QType::CNAME,
            6 => QType::SOA,
            7 => QType::MB,
            8 => QType::MG,
            9 => QType::MR,
            10 => QType::NULL,
            11 => QType::WKS,
            12 => QType::PTR,
            13 => QType::HINFO,
            14 => QType::MINFO,
            15 => QType::MX,
            16 => QType::TXT,
            _ => panic!("Unknown QType: {value}"),
        }
    }
}

impl From<QType> for u16 {
    fn from(qtype: QType) -> Self {
        match qtype {
            QType::A => 1,
            QType::NS => 2,
            QType::MD => 3,
            QType::MF => 4,
            QType::CNAME => 5,
            QType::SOA => 6,
            QType::MB => 7,
            QType::MG => 8,
            QType::MR => 9,
            QType::NULL => 10,
            QType::WKS => 11,
            QType::PTR => 12,
            QType::HINFO => 13,
            QType::MINFO => 14,
            QType::MX => 15,
            QType::TXT => 16,
        }
    }
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum QClass {
    IN,
    CS,
    CH,
    HS,
}

impl From<u16> for QClass {
    fn from(value: u16) -> Self {
        match value {
            1 => QClass::IN,
            2 => QClass::CS,
            3 => QClass::CH,
            4 => QClass::HS,
            _ => panic!("Unknown QClass: {value}"),
        }
    }
}

pub fn parse_questions(payload: &[u8]) -> Result<(Question, usize), String> {
    let mut offset = 0;

    let qname_start = offset;
    while offset < payload.len() && payload[offset] != 0 {
        let label_len = payload[offset] as usize;
        offset += 1 + label_len;

        if offset >= payload.len() {
            return Err("Malformed".to_string());
        }
    }

    if offset >= payload.len() {
        return Err("Malformed".to_string());
    }

    // null terminator
    offset += 1;
    let qname = &payload[qname_start..offset];

    if offset + 4 > payload.len() {
        return Err("too short".to_string());
    }

    let qtype_value = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
    let qtype: QType = qtype_value.into();
    offset += 2;

    let qclass_value = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
    let qclass: QClass = qclass_value.into();
    offset += 2;

    Ok((Question {
        qname,
        qtype,
        qclass,
    }, offset))
}
