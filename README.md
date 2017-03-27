# html2runes

[![Build Status](https://travis-ci.org/spacecowboy/html2runes.svg?branch=master)](https://travis-ci.org/spacecowboy/html2runes)

A HTML to Text converter program written in Rust. Useful to convert html-only emails to plaintext for example.

## Build

```sh
cargo build --release
```

## Run

`html2runes` reads html on STDIN and outputs plaintext (currently only markdown supported) on STDOUT:

```
cat bad.html | target/release/html2runes
```

Result:

```
Text without a paragraph

Text inside paragraph

paragraph again
new line but no ending tags yet and here comes a bull shit token BLA TEXT which should be ignored, and here comes an image ![no alt text](bla.png)invalid image txt
[I am the link text](http://google.com)
```
