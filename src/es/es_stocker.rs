#[cfg(test)]
mod tests;

use crate::ts::TsPacket;
use es::EsBuilder;
use std::collections::HashMap;
use std::convert::TryInto;

pub struct EsStocker<F: FnMut(u16, Vec<u8>)> {
    config: EsStockerConfig,
    builders: HashMap<u16, EsBuilder>,
    func_when_complete_pes: F,
}

pub struct EsStockerConfig {
    pub taking_pids: Vec<u16>,
}

impl<F: FnMut(u16, Vec<u8>)> EsStocker<F> {
    pub fn new(func_when_complete_pes: F, config: EsStockerConfig) -> EsStocker<F> {
        EsStocker {
            config,
            builders: HashMap::new(),
            func_when_complete_pes,
        }
    }

    pub fn set(&mut self, packet: &TsPacket) -> Result<(), &'static str> {
        let pid = packet.header.pid;
        if !self.config.taking_pids.contains(&pid) {
            return Ok(());
        }

        if let Some(builder) = self.builders.get_mut(&pid) {
            if let Ok(building) = builder.push(packet) {
                if !building {
                    (self.func_when_complete_pes)(
                        pid,
                        self.builders.remove(&pid).unwrap().try_into()?,
                    );
                }
            } else {
                panic!("Not implemented")
            }
        } else {
            if let Ok(builder) = EsBuilder::new(packet) {
                self.builders.insert(pid, builder);
            }
        }
        Ok(())
    }
}
