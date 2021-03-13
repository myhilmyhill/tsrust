use std::io::BufRead;

pub struct TsPacket {
    pub reader: std::io::BufReader<std::fs::File>,
}

impl Iterator for TsPacket {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: Vec<u8> = Vec::new();
        let i = self.reader.read_until(b'G', &mut buf);
        match i {
            Ok(count) => {
                if count == 188 {
                    Some(buf)
                } else {
                    self.next()
                }
            }
            Err(_) => None,
        }
    }
}
