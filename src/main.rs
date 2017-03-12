extern crate clap;
extern crate html2runes;

use clap::App;

use html2runes::parse::convert_stdin;

fn main() {

    let args = App::new("html2textrs")
        .version("0.1")
        .about("Converts html to plain text")
        //.arg(Arg::with_name("version")
        //    .short("v")
        //    .long("version")
        //    .help("display the program's version and exit"))
        .get_matches();

    let result = convert_stdin();
    println!("{}", result)
}
