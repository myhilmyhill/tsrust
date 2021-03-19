use super::*;

#[test]
fn reads_tei_as_true() {
    let bytes = [0x80u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert_eq!(true, header.tei);
}

#[test]
fn reads_tei_as_false() {
    let bytes = [0x00u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert_eq!(false, header.tei);
}

#[test]
fn reads_pusi_as_true() {
    let bytes = [0x40u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert_eq!(true, header.pusi);
}

#[test]
fn reads_pusi_as_false() {
    let bytes = [0x00u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert_eq!(false, header.pusi);
}

#[test]
fn reads_tp_as_true() {
    let bytes = [0x20u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert_eq!(true, header.tp);
}

#[test]
fn reads_tp_as_false() {
    let bytes = [0x00u8, 0x00, 0x00];
    let header = TsHeader::from(&bytes[..]);
    assert_eq!(false, header.tp);
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
        assert_eq!(p.expected, header.pid);
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
        assert_eq!(p.expected, header.tsc);
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
        assert_eq!(p.expected, header.afc);
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
        assert_eq!(p.expected, header.cc);
    }
}

#[test]
fn reads_adaptation_field_without_payload() {
    let bytes = [2u8, 0x00, 0x00];
    let adaptation = try_get_adaptation(0b10, &bytes);
    assert!(adaptation.is_some());
    assert_eq!(adaptation.unwrap().length, 3);
}

#[test]
fn reads_adaptation_field_with_payload() {
    let bytes = [1u8, 0x00, 0x00];
    let adaptation = try_get_adaptation(0b11, &bytes);
    assert!(adaptation.is_some());
    assert_eq!(adaptation.unwrap().length, 2);
}

#[test]
fn skips_adaptation_field_without_payload() {
    let bytes = [];
    let adaptation = try_get_adaptation(0b00, &bytes);
    assert!(adaptation.is_none());
}

#[test]
fn skips_adaptation_field_with_payload() {
    let bytes = [];
    let adaptation = try_get_adaptation(0b01, &bytes);
    assert!(adaptation.is_none());
}

#[test]
fn reads_payload_with_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b11, &bytes);
    let payload = try_get_payload(0b11, &bytes, &adaptation);
    assert_eq!(payload.is_some(), true);
    assert_eq!(payload.unwrap().length, 188 - 4 - 1);
}

#[test]
fn reads_payload_without_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b01, &bytes);
    let payload = try_get_payload(0b01, &bytes, &adaptation);
    assert!(payload.is_some());
    assert_eq!(payload.unwrap().length, 188 - 4);
}

#[test]
fn skips_payload_with_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b10, &bytes);
    let payload = try_get_payload(0b10, &bytes, &adaptation);
    assert!(payload.is_none());
}


#[test]
fn skips_payload_without_adaptation_field() {
    let bytes = [0u8; 188 - 4];
    let adaptation = try_get_adaptation(0b00, &bytes);
    let payload = try_get_payload(0b00, &bytes, &adaptation);
    assert!(payload.is_none());
}
