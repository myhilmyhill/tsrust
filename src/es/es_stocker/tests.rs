use super::*;

#[test]
fn stocks_one_pid() -> Result<(), &'static str> {
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

    let mut completed = 0;
    let mut stocker = EsStocker::new(
        |pid, v| {
            println!("{}: {:?}", pid, v);
            completed = completed + 1;
        },
        EsStockerConfig {
            taking_pids: vec![274],
        },
    );
    stocker.set(&packet_1)?;
    stocker.set(&packet_2)?;
    stocker.set(&packet_2)?;
    stocker.set(&packet_2)?;
    stocker.set(&packet_1)?;
    stocker.set(&packet_2)?;
    stocker.set(&packet_2)?;
    stocker.set(&packet_2)?;
    assert_eq!(completed, 2);
    Ok(())
}

#[test]
fn stocks_two_pids() -> Result<(), &'static str> {
    let ts_header_1_a = vec![0x47u8, 0x41, 0x12, 0x10];
    let ts_header_2_a = vec![0x47u8, 0x01, 0x12, 0x11];
    let bytes_1_a = [
        ts_header_1_a,
        vec![
            0x00, 0x00, 0x01, 0x00, // PES packet header
            0x02, 0xda, // PES packet length (730, payload: 720)
            0x80, 0x80, // Optional PES header
            0x07, // PES header length
        ],
        vec![0x00; 188 - 13],
    ]
    .concat();
    let bytes_2_a = [ts_header_2_a, vec![0x00; 188 - 4]].concat(); // 184
    let packet_1_a = TsPacket::try_new(bytes_1_a).unwrap();
    let packet_2_a = TsPacket::try_new(&bytes_2_a[..]).unwrap();

    let ts_header_1_b = vec![0x47u8, 0x41, 0x13, 0x10];
    let ts_header_2_b = vec![0x47u8, 0x01, 0x13, 0x11];
    let bytes_1_b = [
        ts_header_1_b,
        vec![
            0x00, 0x00, 0x01, 0x00, // PES packet header
            0x02, 0xda, // PES packet length (730, payload: 720)
            0x80, 0x80, // Optional PES header
            0x07, // PES header length
        ],
        vec![0x00; 188 - 13],
    ]
    .concat();
    let bytes_2_b = [ts_header_2_b, vec![0x00; 188 - 4]].concat(); // 184
    let packet_1_b = TsPacket::try_new(bytes_1_b).unwrap();
    let packet_2_b = TsPacket::try_new(&bytes_2_b[..]).unwrap();

    let mut completed = 0;
    let mut stocker = EsStocker::new(
        |pid, v| {
            println!("{}: {:?}", pid, v);
            completed = completed + 1;
        },
        EsStockerConfig {
            taking_pids: vec![274, 275],
        },
    );

    // any order
    stocker.set(&packet_1_a)?;
    stocker.set(&packet_1_b)?;
    stocker.set(&packet_2_a)?;
    stocker.set(&packet_2_a)?;
    stocker.set(&packet_2_a)?;
    stocker.set(&packet_2_b)?;
    stocker.set(&packet_2_b)?;
    stocker.set(&packet_2_b)?;
    assert_eq!(completed, 2);
    Ok(())
}
