extern crate clap;
extern crate html2runes;

use clap::{App, Arg};
use html2runes::markdown;
use std::str::FromStr;

enum Format {
    Markdown,
}

impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Format, ()> {
        match s {
            "markdown" => Ok(Format::Markdown),
            _ => Err(()),
        }
    }
}

fn main() {
    let args = App::new("html2textrs")
        .version("0.1")
        .about("Converts html from STDIN to plain text on STDOUT.")
        .arg(Arg::with_name("format")
            .short("f")
            .long("format")
            .help("Plain text format to use")
            .possible_values(&["markdown"])
            .default_value("markdown"))
        .get_matches();

    // Default value exists
    let format = args.value_of("format").unwrap();
    let format = format.parse::<Format>().unwrap();

    let result = match format {
        Format::Markdown => markdown::convert_stdin(),
    };
    println!("{}", result)
}
