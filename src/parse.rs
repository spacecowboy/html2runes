use std::io;
use std::default::Default;
use std::iter::repeat;
use std::string::String;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};


/// Convert HTML which is read from STDIN to plain text
pub fn convert_stdin() {
    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();

    parse_html(dom.document);
}

pub fn parse_html(handle: Handle) {
    let mut queue: Vec<Handle> = Vec::new();
    queue.push(handle);
    while !queue.is_empty() {
        let handle = queue.remove(0);
        let node = handle.borrow();

        match node.node {
            Text(ref text) => println!("#text: {}", text),
            Element(ref name, ref _element, ref attrs) => {
                println!("#element: {}", name.local.to_ascii_lowercase());
            }
            _ => {}
        }

        // Make sure we insert into queue to maintain global ordering
        for child in node.children.iter().rev() {
            queue.insert(0, child.clone());
        }
    }
}
