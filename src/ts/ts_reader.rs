#[cfg(test)]
mod tests;

use crate::rawbytes::RawBytes;
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

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 188];
        let reader = &mut self.reader;

        if reader.read_exact(&mut buf).is_ok() {
            TsPacket::try_new(buf)
        } else {
            dbg!(RawBytes(buf.to_vec()));
                None
            }
        }
    }
