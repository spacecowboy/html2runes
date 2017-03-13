use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, Handle, Node};
use html5ever_atoms::QualName;
use html5ever::Attribute;
use tendril::Tendril;
use tendril;

use std::cell::Ref;
use std::collections::LinkedList;

use traits::HtmlConverter;
use parse::{parse_stdin, parse_string};

pub fn convert_stdin() -> String {
    let dom = parse_stdin();
    convert_html(dom.document)
}

pub fn convert_string(s: &str) -> String {
    let dom = parse_string(s);
    convert_html(dom.document)
}

pub fn convert_html(handle: Handle) -> String {
    let mut converter = MarkdownConverter::new();
    converter.convert_html(handle)
}

pub struct MarkdownConverter<'a> {
    buf: String,
    prefix: LinkedList<&'a str>,
}

impl<'a> MarkdownConverter<'a> {
    pub fn new() -> MarkdownConverter<'a> {
        MarkdownConverter {
            buf: String::new(),
            prefix: LinkedList::new(),
        }
    }

    fn convert_html_into_buffer(&mut self, handle: &Handle) {
        let node = handle.borrow();
        match node.node {
            Comment(_) => {}
            Doctype(_, _, _) => {}
            Text(ref text) => convert_text(text, &mut self.buf, &mut self.prefix),
            Element(ref name, _, ref attrs) => {
                self.handle_element(&name, &attrs, &node);
            }
            Document => {
                for child in node.children.iter() {
                    self.convert_html_into_buffer(&child);
                }
            }
        }
    }

    fn handle_element(&mut self, name: &QualName, attrs: &Vec<Attribute>, node: &Ref<Node>) {
        let name: &str = &name.local.to_ascii_lowercase().to_lowercase();

        match name {
            "head" | "style" | "script" => {
                // ignore these
            }
            _ => {
                // start element
                element_start(&name, &attrs, &mut self.buf, &mut self.prefix);
                // do contents
                for child in node.children.iter() {
                    self.convert_html_into_buffer(&child);
                }
                // end element
                element_end(&name, &attrs, &mut self.buf, &mut self.prefix);
            }
        }
    }
}

impl<'a> HtmlConverter for MarkdownConverter<'a> {
    fn convert_html(&mut self, handle: Handle) -> String {
        self.convert_html_into_buffer(&handle);
        self.buf.clone()
    }
}

fn convert_text(text: &Tendril<tendril::fmt::UTF8>,
                buf: &mut String,
                prefix: &mut LinkedList<&str>) {
    // Start with prefixes
    for p in prefix.iter() {
        buf.push_str(p);
    }

    // Separate prefix(es) from actual text with one space
    if !prefix.is_empty() {
        buf.push_str(" ");
    }

    // True if previous is whitespace
    let mut prev = buf.is_empty() || buf.ends_with(" ") || buf.ends_with("\n");
    for c in text.chars() {
        match c {
            // Stick to a single space
            ' ' | '\n' => {
                if !prev {
                    prev = true;
                    buf.push(' ');
                }
            }
            _ => {
                prev = false;
                buf.push(c);
            }
        }
    }
}

fn element_start(name: &str,
                 attrs: &Vec<Attribute>,
                 buf: &mut String,
                 prefix: &mut LinkedList<&str>) {
    match name {
        "b" | "strong" => bold_start(buf),
        "i" | "em" => emphasize_start(buf),
        "p" | "div" => ensure_double_newline(buf),
        "blockquote" => blockquote_start(buf, prefix),
        "br" => ensure_newline(buf),
        "a" => link_start(buf),
        "img" => img_start(buf, attrs),
        _ => {}
    }
}

fn element_end(name: &str,
               attrs: &Vec<Attribute>,
               buf: &mut String,
               prefix: &mut LinkedList<&str>) {
    match name {
        "b" | "strong" => bold_end(buf),
        "i" | "em" => emphasize_end(buf),
        "blockquote" => blockquote_end(buf, prefix),
        "a" => link_end(buf, attrs),
        _ => {}
    }
}

fn bold_start(buf: &mut String) {
    buf.push_str("**")
}

fn bold_end(buf: &mut String) {
    bold_start(buf)
}

fn emphasize_start(buf: &mut String) {
    buf.push_str("*")
}

fn emphasize_end(buf: &mut String) {
    emphasize_start(buf)
}

fn ensure_double_newline(buf: &mut String) {
    if buf.ends_with("\n\n") {
        // Nothing to do
    } else if buf.ends_with("\n") {
        buf.push_str("\n")
    } else if !buf.is_empty() {
        buf.push_str("\n\n")
    }
}

fn ensure_newline(buf: &mut String) {
    if buf.ends_with("\n") {
        // Nothing to do
    } else if !buf.is_empty() {
        buf.push_str("\n")
    }
}

fn blockquote_start(buf: &mut String, prefix: &mut LinkedList<&str>) {
    ensure_newline(buf);
    prefix.push_back(">")
}

fn blockquote_end(buf: &mut String, prefix: &mut LinkedList<&str>) {
    prefix.pop_back();
    ensure_newline(buf)
}

fn link_start(buf: &mut String) {
    buf.push_str("[")
}

fn link_end(buf: &mut String, attrs: &Vec<Attribute>) {
    let mut url = "";

    for attr in attrs {
        let name: &str = &attr.name.local.to_ascii_lowercase().to_lowercase();
        match name {
            "href" => {
                url = &attr.value;
            }
            _ => {}
        }
    }

    buf.push_str("](");
    buf.push_str(url);
    buf.push_str(")")
}

fn img_start(buf: &mut String, attrs: &Vec<Attribute>) {
    let mut src = "";
    let mut alt = "no alt text";

    for attr in attrs {
        let name: &str = &attr.name.local.to_ascii_lowercase().to_lowercase();
        match name {
            "src" => {
                src = &attr.value;
            }
            "alt" => {
                alt = &attr.value;
            }
            _ => {}
        }
    }

    buf.push_str("![");
    buf.push_str(alt);
    buf.push_str("](");
    buf.push_str(src);
    buf.push_str(")")
}
