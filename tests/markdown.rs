extern crate html2runes;

use html2runes::markdown::*;

#[test]
fn plaintext() {
    let result = convert_string("My little car.");
    assert_eq!("My little car.", result);
}

#[test]
fn newlines_are_ignored() {
    let result = convert_string("My
little
car.");
    assert_eq!("My little car.", result);
}

#[test]
fn lines_with_empty_spaces_are_killed() {
    let result = convert_string("<p>a b c</p>\n <p>d e f</p>");
    assert_eq!("a b c\n\nd e f", result);
}

#[test]
fn bold() {
    let result = convert_string("My <b>little</b> car.");
    assert_eq!("My **little** car.", result);

    let result = convert_string("My <strong>little</strong> car.");
    assert_eq!("My **little** car.", result);
}

#[test]
fn emphasize() {
    let result = convert_string("My <i>little</i> car.");
    assert_eq!("My *little* car.", result);

    let result = convert_string("My <em>little</em> car.");
    assert_eq!("My *little* car.", result);
}

#[test]
fn paragraph() {
    let result = convert_string("<p>A piece of text<br></p><p>Another piece</p>");
    assert_eq!("A piece of text\n\nAnother piece", result);

    let result = convert_string("<p>A piece of text</p>
<p>Another piece</p>");
    assert_eq!("A piece of text\n\nAnother piece", result);

    let result = convert_string("<p>A piece of text<p>Another piece");
    assert_eq!("A piece of text\n\nAnother piece", result);

    let result = convert_string("<div>A piece of text</div><p>Another piece");
    assert_eq!("A piece of text\n\nAnother piece", result);
}

#[test]
fn newline() {
    let result = convert_string("one<br>two<br/>three<br></br>four");
    assert_eq!("one\ntwo\nthree\nfour", result);

    let result = convert_string("one<br><br><br>two");
    assert_eq!("one\ntwo", result);

    let result = convert_string("<br>none");
    assert_eq!("none", result);

}

#[test]
fn blockquote() {
    let result = convert_string("<blockquote>just a quote</blockquote>");
    assert_eq!("> just a quote\n", result);

    let result = convert_string("<blockquote>a nested<blockquote>quote should give \
                                 double</blockquote>lines</blockquote>");
    assert_eq!("> a nested
>> quote should give double
> lines\n",
               result);

    let result = convert_string("<p>And he said:</p><blockquote>Quote me</blockquote>and all was \
                                 good.");
    assert_eq!("And he said:
> Quote me
and all was good.",
               result);

    let result = convert_string("And he said:<blockquote>A long long piece of text<br>which you \
                                 can find in the quote</blockquote>and all was good.");
    assert_eq!("And he said:
> A long long piece of text
> which you can find in the quote
and all was good.",
               result);
}

#[test]
fn link() {
    let result = convert_string("here is a <a href=\"http://google.com\">link</a> to google");
    assert_eq!("here is a [link](http://google.com) to google", result);
}

#[test]
fn image() {
    let result = convert_string("here is an <img alt=\"image\" src=\"bla.png\">");
    assert_eq!("here is an ![image](bla.png)", result);
}

#[test]
fn ignoring_styles() {
    let result = convert_string("should ignore style tag<style>I AM STYLE</style>");
    assert_eq!("should ignore style tag", result);
}

#[test]
fn ignoring_scripts() {
    let result = convert_string("should ignore script tag<script>I AM SCRIPT</script>");
    assert_eq!("should ignore script tag", result);
}

#[test]
fn ignoring_head() {
    let result = convert_string("<html><head><title>I AM HEAD</title></head><body>should ignore \
                                 head tag</body></html>");
    assert_eq!("should ignore head tag", result);
}
