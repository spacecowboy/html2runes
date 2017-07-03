use std::io;
use std::default::Default;

use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::RcDom;

/// Read from stdin and parse as HTML/XML
pub fn parse_stdin() -> RcDom {
    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();
    dom
}

/// Read the provided string ad parse as HTML/XML
pub fn parse_string(text: &str) -> RcDom {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap();
    dom
}
