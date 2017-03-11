use std::io;
use std::io::Error;
use std::default::Default;
use std::iter::repeat;
use std::string::String;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::Attribute;
use html5ever_atoms::QualName;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};


/// Convert HTML which is read from STDIN to plain text
pub fn convert_stdin() -> String {
    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();

    parse_html(dom.document)
}

pub fn convert_string(text: &str) -> String {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap();

    parse_html(dom.document)
}

pub fn parse_html(handle: Handle) -> String {
    let mut result = String::new();
    let mut queue: Vec<Handle> = Vec::new();
    queue.push(handle);
    while !queue.is_empty() {
        let handle = queue.remove(0);
        let node = handle.borrow();

        match node.node {
            Text(ref text) => result += text,
            Element(ref name, ref _element, ref attrs) => {
                handle_element(&name, &attrs, &mut result);
                println!("#element: {}", name.local.to_ascii_lowercase());
            }
            _ => {
                // Make sure we insert into queue to maintain global ordering
                for child in node.children.iter().rev() {
                    queue.insert(0, child.clone());
                }
            }
        }
    }

    result
}

fn handle_element(name: &QualName, attrs: &Vec<Attribute>, result: &mut String) {
    let name: &str = &name.local.to_ascii_lowercase().to_lowercase();

    match name {
        "b" => result.push_str("**"),
        _ => {}
    }
}
