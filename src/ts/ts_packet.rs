#[cfg(test)]
mod tests;

use std::fmt::*;

pub struct RawBytes(pub [u8; 188]);

impl Debug for RawBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for b in &self.0 {
            write!(f, "{:02x} ", b)?;
        }
        write!(f, "\n")
    }
}

pub struct TsPacket {
    pub bytes: RawBytes,
    // pub header: TsHeader,
    // pub adaptation: TsAdaptationField,
    // pub payload: TsPayload,
}

#[derive(Debug)]
pub struct TsHeader {
    pub tei: bool,
    pub pusi: bool,
    pub tp: bool,
    pub pid: u16, // 13 bit
    pub tsc: u8,  // 2 bit
    pub afc: u8,  // 2 bit
    pub cc: u8,   // 4 bit
    pub bytes: Vec<u8>,
}

impl From<Vec<u8>> for TsHeader {
    fn from(bytes: Vec<u8>) -> Self {
        let header = vec![bytes[0], bytes[1], bytes[2]];
        TsHeader {
            bytes,
            tei: header[0] >> 7 == 1,
            pusi: header[0] >> 6 & 1 == 1,
            tp: header[0] >> 5 & 1 == 1,
            pid: (((header[0] & 0x1f) as u16) << 8) as u16 | (header[1]) as u16,
            tsc: header[2] >> 6,
            afc: (header[2] >> 4) & 3,
            cc: header[2] & 0xf,
        }
    }
}

pub struct TsAdaptationField {
    pub bytes: Vec<u8>,
}

pub struct TsPayload {
    pub bytes: Vec<u8>,
}
