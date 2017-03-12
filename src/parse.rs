use std::io;
use std::default::Default;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::RcDom;

use markdown::convert_html;

/// Convert HTML which is read from STDIN to plain text
pub fn convert_stdin() -> String {
    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();

    convert_html(dom.document)
}

pub fn convert_string(text: &str) -> String {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap();

    convert_html(dom.document)
}
