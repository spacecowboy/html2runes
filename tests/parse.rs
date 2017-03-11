extern crate html2runes;

use html2runes::parse::*;

#[test]
fn plaintext() {
    let result = convert_string("My little car.");
    assert_eq!("My little car.", result);
}

#[test]
fn simple_bold() {
    let result = convert_string("My <b>little</br> car.");
    assert_eq!("My **little** car.", result);
}
