#[cfg(test)]
mod tests;

use std::fmt::*;

pub struct RawBytes<T: IntoIterator<Item = u8>>(pub T);

impl<T: IntoIterator<Item = u8> + Clone> Debug for RawBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for b in self.0.clone() {
            write!(f, "{:02x} ", b)?;
        }
        write!(f, "\x08") // backspace
    }
}

#[derive(Debug)]
pub struct TsPacket {
    pub bytes: RawBytes<Vec<u8>>,
    pub header: TsHeader,
    pub adaptation: Option<TsAdaptationField>,
    pub payload: Option<TsPayload>,
}

impl TsPacket {
    pub fn try_new(buf: impl Into<Vec<u8>>) -> Option<Self> {
        let bytes = buf.into();
        if bytes.len() == 188 && bytes[0] == b'G' {
            let header = TsHeader::from(&bytes[1..4]);
            let adaptation = try_get_adaptation(header.afc, &bytes[4..]);
            Some(TsPacket {
                payload: try_get_payload(header.afc, &bytes[4..], &adaptation),
                bytes: RawBytes(bytes),
                header,
                adaptation,
            })
        } else {
            dbg!(RawBytes(bytes));
            None
        }
    }
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
    pub bytes: RawBytes<Vec<u8>>,
}

/// * `bytes` - ommited sync byte
impl From<&[u8]> for TsHeader {
    fn from(header: &[u8]) -> Self {
        TsHeader {
            bytes: RawBytes(header.to_vec()),
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

#[derive(Debug)]
pub struct TsAdaptationField {
    pub length: u8,
    pub bytes: RawBytes<Vec<u8>>,
}

#[derive(Debug)]
pub struct TsPayload {
    pub length: u8,
    pub bytes: RawBytes<Vec<u8>>,
}

/// * `bytes` - ommited header
pub fn try_get_adaptation(afc: u8, bytes: impl Into<Vec<u8>>) -> Option<TsAdaptationField> {
    let bytes = bytes.into();
    if afc >> 1 == 1 {
        let length = bytes[0] + 1;
        Some(TsAdaptationField {
            length,
            bytes: RawBytes(bytes[0..length.into()].to_vec()),
        })
    } else {
        None
    }
}

/// * `bytes` - ommited header
pub fn try_get_payload(
    afc: u8,
    bytes: impl Into<Vec<u8>>,
    adaptation: &Option<TsAdaptationField>,
) -> Option<TsPayload> {
    let bytes = bytes.into();
    if afc & 1 == 1 {
        let len_adaptation = match adaptation {
            Some(e) => e.length,
            None => 0,
        };
        Some(TsPayload {
            length: 188 - 4 - len_adaptation,
            bytes: RawBytes(bytes[len_adaptation.into()..].to_vec()),
        })
    } else {
        None
    }
}
