use super::*;

#[test]
fn reads_packet() {
    let bytes = [vec![0x47u8, 0x21, 0x00, 0x00], vec![0x00u8; 184]].concat();
    let packet = TsPacket::try_new(bytes);
    assert!(packet.is_some());
}

#[test]
fn error_when_wrong_sync_byte() {
    let bytes = [0x00u8; 188];
    let packet = TsPacket::try_new(bytes);
    assert!(packet.is_none());
}

#[test]
fn error_when_wrong_length() {
    let bytes = [vec![0x47u8, 0x21, 0x00, 0x00], vec![0x00u8; 183]].concat();
    let packet = TsPacket::try_new(bytes);
    assert!(packet.is_none());
}

#[test]
fn reads_tei_as_true() {
    let bytes = [0x80u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert!(header.tei);
}

#[test]
fn reads_tei_as_false() {
    let bytes = [0x00u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert!(!header.tei);
}

#[test]
fn reads_pusi_as_true() {
    let bytes = [0x40u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert!(header.pusi);
}

#[test]
fn reads_pusi_as_false() {
    let bytes = [0x00u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert!(!header.pusi);
}

#[test]
fn reads_tp_as_true() {
    let bytes = [0x20u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert!(header.tp);
}

#[test]
fn reads_tp_as_false() {
    let bytes = [0x00u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert!(!header.tp);
}

#[test]
fn reads_pid() {
    struct Param {
        bytes: [u8; 3],
        expected: u16,
    }
    for p in vec![
        Param {
            bytes: [0x00u8, 0x00, 0x00],
            expected: 0,
        },
        Param {
            bytes: [0x1fu8, 0xff, 0x00],
            expected: 0x1fff,
        },
    ] {
        let header = TsHeader::from(&p.bytes[..]);
        assert_eq!(header.pid, p.expected);
    }
}

#[test]
fn reads_tsc() {
    struct Param {
        bytes: [u8; 3],
        expected: u8,
    }
    for p in vec![
        Param {
            bytes: [0x00u8, 0x00, 0x00],
            expected: 0,
        },
        Param {
            bytes: [0x00u8, 0x00, 0xc0],
            expected: 0b11,
        },
        Param {
            bytes: [0x00u8, 0x00, 0x80],
            expected: 0b10,
        },
        Param {
            bytes: [0x00u8, 0x00, 0x40],
            expected: 0b01,
        },
    ] {
        let header = TsHeader::from(&p.bytes[..]);
        assert_eq!(header.tsc, p.expected);
    }
}

#[test]
fn reads_afc() {
    struct Param {
        bytes: [u8; 3],
        expected: u8,
    }
    for p in vec![
        Param {
            bytes: [0x00u8, 0x00, 0x00],
            expected: 0,
        },
        Param {
            bytes: [0x00u8, 0x00, 0x30],
            expected: 0b11,
        },
        Param {
            bytes: [0x00u8, 0x00, 0x20],
            expected: 0b10,
        },
        Param {
            bytes: [0x00u8, 0x00, 0x10],
            expected: 0b01,
        },
    ] {
        let header = TsHeader::from(&p.bytes[..]);
        assert_eq!(header.afc, p.expected);
    }
}

#[test]
fn reads_cc() {
    struct Param {
        bytes: [u8; 3],
        expected: u8,
    }
    for p in vec![
        Param {
            bytes: [0x00u8, 0x00, 0x00],
            expected: 0,
        },
        Param {
            bytes: [0x00u8, 0x00, 0x0f],
            expected: 0xf,
        },
    ] {
        let header = TsHeader::from(&p.bytes[..]);
        assert_eq!(header.cc, p.expected);
    }
}

#[test]
fn reads_adaptation_field_without_payload() {
    let bytes = [2u8, 0x00, 0x00];
    let adaptation = try_get_adaptation(0b10, bytes);
    assert!(adaptation.is_some());
    assert_eq!(adaptation.unwrap().length, 3);
}

#[test]
fn reads_adaptation_field_with_payload() {
    let bytes = [1u8, 0x00, 0x00];
    let adaptation = try_get_adaptation(0b11, bytes);
    assert!(adaptation.is_some());
    assert_eq!(adaptation.unwrap().length, 2);
}

#[test]
fn skips_adaptation_field_without_payload() {
    let bytes = [];
    let adaptation = try_get_adaptation(0b00, bytes);
    assert!(adaptation.is_none());
}

#[test]
fn skips_adaptation_field_with_payload() {
    let bytes = [];
    let adaptation = try_get_adaptation(0b01, bytes);
    assert!(adaptation.is_none());
}

#[test]
fn reads_payload_with_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b11, bytes);
    let payload = try_get_payload(0b11, bytes, &adaptation);
    assert_eq!(payload.is_some(), true);
    assert_eq!(payload.unwrap().length, 188 - 4 - 1);
}

#[test]
fn reads_payload_without_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b01, bytes);
    let payload = try_get_payload(0b01, bytes, &adaptation);
    assert!(payload.is_some());
    assert_eq!(payload.unwrap().length, 188 - 4);
}

#[test]
fn skips_payload_with_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b10, bytes);
    let payload = try_get_payload(0b10, bytes, &adaptation);
    assert!(payload.is_none());
}

#[test]
fn skips_payload_without_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b00, bytes);
    let payload = try_get_payload(0b00, bytes, &adaptation);
    assert!(payload.is_none());
}
