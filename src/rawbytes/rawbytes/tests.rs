use super::*;

#[test]
fn debug_no_values() {
    assert_eq!(format!("{:?}", RawBytes(vec![])), "\x08")
}

#[test]
fn debug_one_value() {
    assert_eq!(format!("{:?}", RawBytes(vec![0xffu8])), "ff \x08")
}

#[test]
fn debug_two_values() {
    assert_eq!(format!("{:?}", RawBytes(vec![0xffu8, 0xff])), "ff ff \x08")
}
