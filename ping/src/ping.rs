use crate::prelude::*;

#[derive(Serialize, Debug, Copy, Clone)]
pub struct Icmp {
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub icmp_checksum: [u8; 2],
    pub icmp_ident: u16,
    pub icmp_seq: u16,
    pub data: [u8; 8],
}

impl Icmp {
    pub fn new() -> Self {
        Self {
            icmp_type: 0,
            icmp_code: 0,
            icmp_checksum: [0; 2],
            icmp_ident: 0,
            icmp_seq: 0,
            data: [0; 8],
        }
    }
}

pub fn cksum_calc(data: &mut Vec<u8>) -> &mut Vec<u8> {
    let mut i = 0;
    let mut checksum: u32 = 0;
    while i + 2 <= data.len() {
        checksum += u16::from_le_bytes(data[i..i + 2].try_into().unwrap()) as u32;

        i += 2;
    }

    while checksum > 0xffff {
        checksum = (checksum & 0xffff) + (checksum >> 2 * 8);
    }

    let mut checksum = checksum as u16;

    checksum = !checksum & 0xffff;

    // endianness
    //checksum = checksum >> 8 | ((checksum & 0xff) << 8);

    data[2] = (checksum & 0xff) as u8;
    data[3] = (checksum >> 8) as u8;

    data
}
