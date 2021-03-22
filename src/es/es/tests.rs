use super::*;

#[test]
fn reads_typical_pes() -> Result<(), &'static str> {
    let ts_header_1 = vec![0x47u8, 0x41, 0x12, 0x10];
    let ts_header_2 = vec![0x47u8, 0x01, 0x12, 0x11];
    let bytes_1 = [
        ts_header_1,
        vec![
            0x00, 0x00, 0x01, 0x00, // PES packet header
            0x02, 0xda, // PES packet length (730, payload: 720)
            0x80, 0x80, // Optional PES header
            0x07, // PES header length
        ],
        vec![0x00; 188 - 13],
    ]
    .concat();
    let bytes_2 = [ts_header_2, vec![0x00; 188 - 4]].concat(); // 184
    let packet_1 = TsPacket::try_new(bytes_1).unwrap();
    let packet_2 = TsPacket::try_new(&bytes_2[..]).unwrap();

    let mut builder = EsBuilder::new(&packet_1)?; // 168
    assert!(builder.push(&packet_2).unwrap()); // 352
    assert!(builder.push(&packet_2).unwrap()); // 536
    assert!(!builder.push(&packet_2).unwrap()); // 720
    assert!(builder.push(&packet_2).is_err());
    let es: Vec<u8> = builder.try_into()?;
    assert_eq!(es.len(), 730 - 3 - 7);
    Ok(())
}

#[test]
fn reads_one_byte_stuffing_pes() -> Result<(), &'static str> {
    let ts_header_1 = vec![0x47u8, 0x41, 0x12, 0x10];
    let ts_header_2 = vec![0x47u8, 0x01, 0x12, 0x11];
    let bytes_1 = [
        ts_header_1,
        vec![
            0x00, 0x00, 0x01, 0x00, // PES packet header
            0x02, 0xd9, // PES packet length (729, payload: 719)
            0x80, 0x80, // Optional PES header
            0x07, // PES header length
        ],
        vec![0x00; 188 - 13],
    ]
    .concat();
    let bytes_2 = [ts_header_2, vec![0x00; 188 - 4]].concat(); // 184
    let packet_1 = TsPacket::try_new(bytes_1).unwrap();
    let packet_2 = TsPacket::try_new(&bytes_2[..]).unwrap();

    let mut builder = EsBuilder::new(&packet_1)?;
    assert!(builder.push(&packet_2).unwrap());
    assert!(builder.push(&packet_2).unwrap());
    assert!(!builder.push(&packet_2).unwrap());
    assert!(builder.push(&packet_2).is_err());
    let es: Vec<u8> = builder.try_into()?;
    assert_eq!(es.len(), 729 - 3 - 7);
    Ok(())
}

#[test]
fn reads_183_bytes_stuffing_pes() -> Result<(), &'static str> {
    let ts_header_1 = vec![0x47u8, 0x41, 0x12, 0x10];
    let ts_header_2 = vec![0x47u8, 0x01, 0x12, 0x11];
    let bytes_1 = [
        ts_header_1,
        vec![
            0x00, 0x00, 0x01, 0x00, // PES packet header
            0x02, 0x23, // PES packet length (547, payload: 537)
            0x80, 0x80, // Optional PES header
            0x07, // PES header length
        ],
        vec![0x00; 188 - 13],
    ]
    .concat();
    let bytes_2 = [ts_header_2, vec![0x00; 188 - 4]].concat(); // 184
    let packet_1 = TsPacket::try_new(bytes_1).unwrap();
    let packet_2 = TsPacket::try_new(&bytes_2[..]).unwrap();

    let mut builder = EsBuilder::new(&packet_1)?;
    assert!(builder.push(&packet_2).unwrap());
    assert!(builder.push(&packet_2).unwrap());
    assert!(!builder.push(&packet_2).unwrap());
    assert!(builder.push(&packet_2).is_err());
    let es: Vec<u8> = builder.try_into()?;
    assert_eq!(es.len(), 547 - 3 - 7);
    Ok(())
}
