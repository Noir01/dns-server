#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DNSHeader {
    pub fn from_bytes(header: &[u8]) -> Self {
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

    pub fn new(id: u16) -> Self {
        DNSHeader {
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