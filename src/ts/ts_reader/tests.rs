use super::TsReader;
use std::io::Cursor;
use std::iter;
use File;

#[test]
fn drops_loss_packet() {
    let bytes = vec![b'G'];
    let cursor = Cursor::new(bytes);
    let mut header = TsReader::new(cursor);
    let result = header.next();
    assert!(result.is_none());
}

#[test]
fn drops_unsync_packet() {
    let bytes: Vec<u8> = iter::repeat(b'0').take(188).collect();
    let cursor = Cursor::new(bytes);
    let mut header = TsReader::new(cursor);
    let result = header.next();
    assert!(result.is_none());
}

#[test]
fn reads_one_packet() {
    let bytes: Vec<u8> = iter::repeat(b'G').take(188).collect();
    let cursor = Cursor::new(&bytes);
    let mut header = TsReader::new(cursor);

    let result = header.next();
    assert!(result.is_some());
    assert!(result.unwrap().bytes.0.iter().eq(bytes.iter()));

    let result = header.next();
    assert!(result.is_none());
}

#[test]
fn reads_packets_over_buffer_size() {
    let mut packet = [0u8; 188];
    packet[0] = b'G';

    let n = 10000;
    let bytes: Vec<u8> = packet.iter().cloned().cycle().take(188 * n).collect();
    let cursor = Cursor::new(bytes);
    let mut header = TsReader::new(cursor);

    for _ in 0..n {
        let result = header.next();
        assert!(result.is_some());
        assert!(result.unwrap().bytes.0.iter().eq(packet.iter()));
    }

    let result = header.next();
    assert!(result.is_none());
}

#[test]
#[ignore]
fn reads_file() -> std::io::Result<()> {
    let file = File::open("test.ts")?;
    let reader = TsReader::new(file);
    for _ in reader {}
    Ok(())
}
