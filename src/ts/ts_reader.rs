#[cfg(test)]
mod tests;

use super::ts_packet::RawBytes;
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
            Some(TsPacket {
                bytes: RawBytes(buf.to_vec()),
                header: buf[1..4].into(),
            })
        } else {
            dbg!(RawBytes(buf.to_vec()));
            None
        }
    }
}
