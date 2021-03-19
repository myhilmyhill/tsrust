#[cfg(test)]
mod tests;

use std::fmt::*;

pub struct RawBytes<T: IntoIterator<Item = u8>>(pub T);

impl<T: IntoIterator<Item = u8> + Clone> Debug for RawBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for b in self.0.clone() {
            write!(f, "{:02x} ", b)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TsPacket {
    pub bytes: RawBytes<Vec<u8>>,
    pub header: TsHeader,
    // pub adaptation: Option<TsAdaptationField>,
    // pub payload: Option<TsPayload>,
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

// #[derive(Debug)]
// pub struct TsAdaptationField {
//     pub length: u8,
//     pub bytes: RawBytes<Vec<u8>>,
// }

// #[derive(Debug)]
// pub struct TsPayload {
//     pub length: u8,
//     pub bytes: RawBytes<Vec<u8>>,
// }

// fn try_get_adaptation(afc: u8, bytes: &Vec<u8>) -> Option<TsAdaptationField> {
//     if afc >> 1 == 1 {
//         let length = bytes[3];
//         Some(TsAdaptationField {
//             length,
//             bytes: RawBytes(bytes[3..length.into()].to_vec()),
//         })
//     } else {
//         None
//     }
// }

// fn try_get_payload(
//     afc: u8,
//     bytes: &Vec<u8>,
//     adaptation: &Option<TsAdaptationField>,
// ) -> Option<TsPayload> {
//     match afc {
//         0b11 => Some(TsPayload {
//             length: 188 - 24 - adaptation.unwrap().length,
//             bytes: RawBytes(bytes[adaptation.unwrap().length.into()..].to_vec()),
//         }),
//         0b01 => Some(TsPayload {
//             length: 188 - 24,
//             bytes: RawBytes(bytes[3..].to_vec()),
//         }),
//     }
// }
