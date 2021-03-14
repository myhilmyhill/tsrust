use crate::ts::TsPacket;
use std::io::BufRead;

pub struct TsReader {
    pub reader: std::io::BufReader<std::fs::File>,
}

impl Iterator for TsReader {
    type Item = TsPacket;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: Vec<u8> = Vec::new();
        let i = self.reader.read_until(b'G', &mut buf);
        match i {
            Ok(count) => {
                if count == 188 {
                    Some(TsPacket { bytes: buf })
                } else {
                    self.next()
                }
            }
            Err(_) => None,
        }
    }
}
