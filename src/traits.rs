use std::collections::LinkedList;

pub trait HtmlConverter {
    fn convert_html_into_buffer(&self, buf: &mut String, prefix: &mut LinkedList<&str>);
}
