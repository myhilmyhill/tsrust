#[cfg(test)]
mod tests;

use crate::ts::TsPacket;
use std::convert::TryInto;

pub struct EsBuilder {
    length: u16,
    buffer: Vec<u8>,
    pid: u16,
    stream_id: u8,
    current_cc: u8,
}

impl EsBuilder {
    /// must have pes packet header
    pub fn new(ts_packet: &TsPacket) -> Result<Self, &'static str> {
        if !ts_packet.header.pusi {
            return Err("pusi != 1");
        }
        let pid = ts_packet.header.pid;
        let current_cc = ts_packet.header.cc;
        let packet = &ts_packet.payload.as_ref().ok_or("No payload")?.bytes.0;
        if !(packet[0] == 0x00u8 && packet[1] == 0x00u8 && packet[2] == 0x01u8) {
            return Err("Not PES");
        }
        let stream_id = packet[3];
        let length = (packet[4] as u16) << 8 | packet[5] as u16;
        let mut buffer = Vec::new();
        if packet[6] >> 6 == 0b10 {
            let len_remainder = packet[5 + 3] as u16;
            if length == 0 {
                return Err("Not implemented for PES length == 0");
            } else if length < 3 || len_remainder > length - 3 {
                return Err("Wrong PES length");
            }

            buffer.append(&mut packet[(5 + 3 + 1 + len_remainder).into()..].to_vec());
            Ok(EsBuilder {
                length: length - 3 - len_remainder,
                buffer,
                pid,
                stream_id,
                current_cc,
            })
        } else {
            // no optional PES header
            buffer.append(&mut packet[5..].to_vec());
            Ok(EsBuilder {
                length,
                buffer,
                pid,
                stream_id,
                current_cc,
            })
        }
    }

    /// true -> building PES
    /// false -> complete PES
    pub fn push(&mut self, ts_packet: &TsPacket) -> Result<bool, &'static str> {
        if ts_packet.header.pusi {
            return Err("pusi == 1");
        }
        if ts_packet.header.pid != ts_packet.header.pid {
            return Err("Wrong pid");
        }
        if ((ts_packet.header.cc as u16 + 1u16) & 0x0f) as u8 == ts_packet.header.cc {
            return Err("Wrong cc due to drop packet");
        }
        let adding = &ts_packet.payload.as_ref().ok_or("No payload")?.bytes.0;
        let end: i32 = (adding.len() + self.buffer.len()) as i32 - self.length as i32;
        if end < 0 {
            // uncomplete
            self.buffer.append(&mut adding.to_vec());
            Ok(true)
        } else if end < 184 {
            // to complete (if neccesary cut overflow bytes)
            self.buffer
                .append(&mut adding[..(184 - end) as usize].to_vec());
            Ok(false)
        } else {
            Err("No more packets")
        }
    }
}

impl TryInto<Vec<u8>> for EsBuilder {
    type Error = &'static str;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if self.buffer.len() == self.length.into() {
            Ok(self.buffer)
        } else {
            Err("Not complete PES")
        }
    }
}
