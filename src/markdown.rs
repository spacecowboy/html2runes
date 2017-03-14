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
    list_markers: Vec<Option<usize>>,
}

impl<'a> MarkdownConverter<'a> {
    pub fn new() -> MarkdownConverter<'a> {
        MarkdownConverter {
            buf: String::new(),
            prefix: LinkedList::new(),
            list_markers: Vec::new(),
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
                self.element_start(&name, &attrs);
                // do contents
                for child in node.children.iter() {
                    self.convert_html_into_buffer(&child);
                }
                // end element
                self.element_end(&name, &attrs);
            }
        }
    }

    fn element_start(&mut self,
                     name: &str,
                     attrs: &Vec<Attribute>) {
        match name {
            "b" | "strong" => bold_start(&mut self.buf),
            "i" | "em" => emphasize_start(&mut self.buf),
            "p" | "div" => self.p_start(),
            "blockquote" => blockquote_start(&mut self.buf, &mut self.prefix),
            "br" => self.br_start(),
            "a" => link_start(&mut self.buf),
            "img" => img_start(&mut self.buf, attrs),
            "ul" => ul_start(&mut self.buf, &mut self.list_markers),
            "li" => li_start(&mut self.buf, &mut self.list_markers),
            _ => {}
        }
    }

    fn element_end(&mut self,
                   name: &str,
                   attrs: &Vec<Attribute>) {
        match name {
            "b" | "strong" => bold_end(&mut self.buf),
            "i" | "em" => emphasize_end(&mut self.buf),
            "blockquote" => blockquote_end(&mut self.buf, &mut self.prefix),
            "a" => link_end(&mut self.buf, attrs),
            "ul" => ul_end(&mut self.buf, &mut self.list_markers),
            "li" => {
                li_end(&mut self.buf, &mut self.list_markers);
            },
            _ => {}
        }
    }

    fn p_start(&mut self) {
        if let Some(prefix) = prefix(&self.list_markers) {
            if self.buf.ends_with(&prefix) {
                return;
            }
        }
        ensure_double_newline(&mut self.buf);
        prefix_with_necessary_spaces(&mut self.buf, &self.list_markers);
    }

    fn br_start(&mut self) {
        if let Some(prefix) = prefix(&self.list_markers) {
            if self.buf.ends_with(&prefix) {
                return;
            }
        }
        ensure_newline(&mut self.buf);
        prefix_with_necessary_spaces(&mut self.buf, &self.list_markers);
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

fn trim_ending_whitespace(buf: &mut String) {
    while buf.ends_with(" ") || buf.ends_with("\t") {
        let end = buf.len() - 1;
        buf.remove(end);
    }
}

fn prefix(list_markers: &Vec<Option<usize>>) -> Option<String> {
    if let Some(mark) = list_markers.last() {
        match *mark {
            Some(i) => Some(format!("{}. ", i)),
            None => Some("* ".to_string()),
        }
    } else {
        None
    }
}

fn prefix_with_necessary_spaces(buf: &mut String, list_markers: &[Option<usize>]) {
    let count = list_markers.iter().fold(0, |sum,mark| {
        match *mark {
            Some(_) => sum + 3, // '1. ' = three characters
            None => sum + 2, // '* ' = two characters
        }
    });

    buf.push_str(&(0..count).map(|_| " ").collect::<String>());
}

fn ensure_double_newline(buf: &mut String) {
    trim_ending_whitespace(buf);
    if buf.ends_with("\n\n") {
        // Nothing to do
    } else if buf.ends_with("\n") {
        buf.push_str("\n")
    } else if !buf.is_empty() {
        buf.push_str("\n\n")
    }
}

fn ensure_newline(buf: &mut String) {
    trim_ending_whitespace(buf);
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

fn ul_start(buf: &mut String, list_markers: &mut Vec<Option<usize>>) {
    ensure_double_newline(buf);
    list_markers.push(None);
}

fn ul_end(buf: &mut String, list_markers: &mut Vec<Option<usize>>) {
    ensure_double_newline(buf);
    list_markers.pop();
    prefix_with_necessary_spaces(buf, &list_markers);
}

fn li_start(buf: &mut String, list_markers: &Vec<Option<usize>>) {
    if !list_markers.is_empty() {
        let last_index = list_markers.len() - 1;
        prefix_with_necessary_spaces(buf, list_markers.split_at(last_index).0);
        if let Some(prefix) = prefix(list_markers) {
            buf.push_str(&prefix);
        }
    }
}

fn li_end(buf: &mut String, list_markers: &mut Vec<Option<usize>>) {
    if let Some(mark) = list_markers.pop() {
        ensure_newline(buf);
        match mark {
            Some(i) => list_markers.push(Some(i + 1)),
            None => list_markers.push(mark),
        }
    }
}

#[test]
fn test_prefix_with_necessary_spaces() {
    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[]);
    assert_eq!("", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[None]);
    assert_eq!("  ", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[Some(3)]);
    assert_eq!("   ", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[Some(1), None, Some(2)]);
    assert_eq!("        ", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf,
                                 &[Some(1), None, Some(2)].split_at(2).0);
    assert_eq!("     ", &buf);
}
