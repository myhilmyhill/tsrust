pub struct TsPacket {
    pub bytes: Vec<u8>,
    // pub header: Vec<u8>,
    // pub payload: Vec<u8>,
}

use std::fmt::*;

impl Debug for TsPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for b in &self.bytes {
            write!(f, "{:02x} ", b);
        }
        write!(f, "\n")
    }
}
