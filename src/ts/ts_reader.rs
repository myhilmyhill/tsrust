#[cfg(test)]
mod tests;

use super::ts_packet::try_get_adaptation;
use super::ts_packet::try_get_payload;
use super::ts_packet::RawBytes;
use super::TsHeader;
use crate::ts::TsPacket;
use std::io::Read;
use BufReader;

pub struct TsReader<R> {
    pub reader: BufReader<R>,
}

impl<R: Read> TsReader<R> {
    pub fn new(inner: R) -> TsReader<R> {
        TsReader {
            reader: BufReader::new(inner),
        }
    }
}

impl<R: Read> Iterator for TsReader<R> {
    type Item = TsPacket;

    fn next(&'_ mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 188];
        let reader = &mut self.reader;

        if reader.read_exact(&mut buf).is_ok() && buf[0] == b'G' {
            let header = TsHeader::from(&buf[1..4]);
            let adaptation = try_get_adaptation(header.afc, &buf[4..]);
            Some(TsPacket {
                bytes: RawBytes(buf.to_vec()),
                payload: try_get_payload(header.afc, &buf[4..], &adaptation),
                header,
                adaptation,
            })
        } else {
            dbg!(RawBytes(buf.to_vec()));
            None
        }
    }
}
