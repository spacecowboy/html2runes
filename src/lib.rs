extern crate html5ever;

pub mod parse;
pub mod traits;
pub mod markdown;

pub fn html_to_text(html: &str) -> String {
    markdown::convert_string(html)
}
