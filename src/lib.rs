extern crate html5ever_atoms;
extern crate html5ever;
extern crate tendril;

pub mod parse;
pub mod traits;
pub mod markdown;

pub fn html_to_text(html: &str) -> String {
    markdown::convert_string(html)
}
