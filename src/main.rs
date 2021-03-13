use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


struct TsPacket {
    reader: std::io::BufReader<std::fs::File>,
}

impl Iterator for TsPacket {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        let i = self.reader.read_until(b'G', &mut buf);
        match i {
            Ok(_) => Some(buf),
            Err(_) => None,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        fn a(reader: std::io::BufReader<std::fs::File>) -> TsPacket {
            TsPacket { reader }
        }
        for i in a(reader) {
            println!("{:?}", i);
        }
    }
    Ok(())
}