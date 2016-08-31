extern crate postcss;
use postcss::tokenizer::{
    Location,
    Token,
    Tokenizer
};

fn test(input:&str, expected:Vec<Token>) {
    assert_eq!(
        Tokenizer::new(input).get_tokens(),
        &expected
    );
}

#[test]
fn it_compiles() {
    test("", vec![]);
}

#[test]
fn it_tokenizes_single_character_space() {
    test(" ", vec![Token::Space(" ".to_string())]);
    test(&'\n'.to_string(), vec![Token::Space('\n'.to_string())]);
    test(&'\r'.to_string(), vec![Token::Space('\r'.to_string())]);
    test(&'\t'.to_string(), vec![Token::Space('\t'.to_string())]);
}

#[test]
fn it_tokenizes_space() {
    let s = "\r\n \t";
    test(s, vec![Token::Space(s.to_string())]);
}

#[test]
fn it_tokenizes_word_new() {
    let s = "ab";
    test(s, vec![Token::Word(s.to_string(), Location(1, 1), Location(1, 2))]);
}

#[test]
fn it_splits_word_by_bang() {
    test("aa!bb", vec![
        Token::Word("aa".to_string(), Location(1, 1), Location(1, 2)),
        Token::Word("!bb".to_string(), Location(1, 3), Location(1, 5)),
    ]);
}

#[test]
fn it_changes_lines_in_spaces() {
    test("a \n b", vec![
        Token::Word("a".to_string(), Location(1, 1), Location(1, 1)),
        Token::Space(" \n ".to_string()),
        Token::Word("b".to_string(), Location(2, 2), Location(2, 2)),
    ]);
}

#[test]
fn it_tokenizes_control_chars() {
    test("{:;}", vec![
        Token::Control("{".to_string(), Location(1, 1)),
        Token::Control(":".to_string(), Location(1, 2)),
        Token::Control(";".to_string(), Location(1, 3)),
        Token::Control("}".to_string(), Location(1, 4)),
    ]);
}

#[test]
#[ignore]
fn it_escapes_control_symbols() {
    test("\\(\\{\"", vec![
        Token::Word("\\(".to_string(), Location(1, 1), Location(1, 2)),
        Token::Word("\\{".to_string(), Location(1, 3), Location(1, 4)),
        Token::Word("\"".to_string(), Location(1, 5), Location(1, 6)),
        Token::Word("\\@".to_string(), Location(1, 7), Location(1, 8)),
        Token::Word("\\\\".to_string(), Location(1, 9), Location(1, 10)),
        Token::String("\"\"".to_string(), Location(1, 11), Location(1, 12)),
    ]);
}

#[test]
fn it_escapes_backslash() {
    test("\\\\\\\\{", vec![
        Token::Word("\\\\\\\\".to_string(), Location(1, 1), Location(1, 4)),
        Token::Control("{".to_string(), Location(1, 5)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_simple_brackets() {
    let s = "(ab)";
    test(s, vec![Token::Brackets(s.to_string(), Location(1, 1), Location(1, 4))]);
}

#[test]
#[ignore]
fn it_tokenizes_complicated_brackets() {
    test("(())(\"\")(/**/)(\\\\)(\n)(", vec![
            Token::LeftParen(Location(1, 1)),
            Token::Brackets("()".to_string(), Location(1, 2), Location(1, 3)),
            Token::RightParen(Location(1, 4)),
            Token::LeftParen(Location(1, 5)),
            Token::String("".to_string(), Location(1, 6), Location(1, 7)),
            Token::RightParen(Location(1, 8)),
            Token::LeftParen(Location(1, 9)),
            Token::Comment("/**/".to_string(), Location(1, 10), Location(1, 13)),
            Token::RightParen(Location(1, 14)),
            Token::LeftParen(Location(1, 15)),
            Token::Word("\\\\".to_string(), Location(1, 16), Location(1, 17)),
            Token::Word("\\\\".to_string(), Location(1, 16), Location(1, 17)),
            Token::RightParen(Location(1, 18)),
            Token::RightParen(Location(1, 19)),
            Token::Space("\n".to_string()),
            Token::RightParen(Location(2, 1)),
            Token::LeftParen(Location(2, 2)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_string() {
    test("'\"'\"", vec![
            Token::String("\"'\"".to_string(), Location(1, 1), Location(1, 3)),
            Token::String("\"\\\"".to_string(), Location(1, 4), Location(1, 7)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_escaped_string() {
    test("\"\\\\\"", vec![
            Token::String("\"\\\\\"".to_string(), Location(1, 1), Location(1, 4)),
    ]);
}

#[test]
#[ignore]
fn it_changes_lines_in_strings() {
    test("\"\n\n\"\"\n\n\"", vec![
            Token::String("\"\n\n\"".to_string(), Location(1, 1), Location(3, 1)),
            Token::String("\"\n\n\"".to_string(), Location(3, 2), Location(5, 1)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_at_word() {
    test("@word ", vec![
            Token::AtWord("@word".to_string(), Location(1, 1), Location(1, 5)),
            Token::Space(" ".to_string()),
    ]);
}

#[test]
#[ignore]
fn tokenizes_at_word_end() {
    test("@one{@two()@three\"\"@four;", vec![
        Token::AtWord("@one".to_string(), Location(1, 1), Location(1, 4)),
        Token::Control("{".to_string(), Location(1, 5)),
        Token::AtWord("@two".to_string(), Location(1, 6), Location(1, 9)),
        Token::Brackets("()".to_string(), Location(1, 10), Location(1, 11)),
        Token::AtWord("@three".to_string(), Location(1, 12), Location(1, 17)),
        Token::String("\"\"".to_string(), Location(1, 18), Location(1, 19)),
        Token::AtWord("@four".to_string(), Location(1, 20), Location(1, 24)),
        Token::Control(";".to_string(), Location(1, 25)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_urls() {
    test("url(/*\\))", vec![
        Token::Word("url".to_string(), Location(1, 1), Location(1, 3)),
        Token::Brackets("(/*\\))".to_string(), Location(1, 4), Location(1, 9)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_quoted_urls() {
    test("url(\")\")", vec![
        Token::Word("url".to_string(), Location(1, 1), Location(1, 3)),
        Token::LeftParen(Location(1, 4)),
        Token::String("\")\"".to_string(), Location(1, 5), Location(1, 7)),
        Token::RightParen(Location(1, 8)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_at_symbol() {
    test("@", vec![Token::AtWord("@".to_string(), Location(1, 1), Location(1, 1))]);
}

#[test]
#[ignore]
fn it_tokenizes_comment() {
    test("/* a\nb */", vec![Token::Comment("/* \nb */".to_string(), Location(1, 1), Location(2, 4))]);
}

#[test]
#[ignore]
fn it_changes_lines_in_comments() {
    test("a/* \n */b", vec![
            Token::Word("a".to_string(), Location(1, 1), Location(1, 1)),
            Token::Comment("/* \n */".to_string(), Location(1, 2), Location(2, 3)),
            Token::Word("b".to_string(), Location(2, 4), Location(2, 4)),
    ]);
}

#[test]
#[ignore]
fn it_supports_line_feed() {
    test("a\\fb", vec![
            Token::Word("a".to_string(), Location(1, 1), Location(1, 1)),
            Token::Space("\\f".to_string()),
            Token::Word("b".to_string(), Location(2, 1), Location(2, 1)),
    ]);
}

#[test]
#[ignore]
fn it_supports_carriage_return() {
    test("a\rb\r\nc", vec![
            Token::Word("a".to_string(), Location(1, 1), Location(1, 1)),
            Token::Space("\r".to_string()),
            Token::Word("b".to_string(), Location(2, 1), Location(2, 1)),
            Token::Space("\r\n".to_string()),
            Token::Word("c".to_string(), Location(3, 1), Location(3, 1)),
    ]);
}

#[test]
#[ignore]
fn it_tokenizes_css() {
    let css = concat!(
        "a {\n",
        "  content: \"a\";\n",
        "  width: calc(1px;)\n",
        "  }\n",
        "/* small screen */\n",
        "@media screen {}"
    );
    test(css, vec![
        Token::Word("a".to_string(), Location(1, 1), Location(1, 1)),
        Token::Space(" ".to_string()),
        Token::Control("{".to_string(), Location(1, 3)),
        Token::Space("\n  ".to_string()),
        Token::Word("content".to_string(), Location(2, 3), Location(2, 9)),
        Token::Control(":".to_string(), Location(2, 10)),
        Token::Space(" ".to_string()),
        Token::Word("a".to_string(), Location(2, 12), Location(2, 14)),
        Token::Control(";".to_string(), Location(2, 15)),
        Token::Space("\n  ".to_string()),
        Token::Word("width".to_string(), Location(3, 3), Location(3, 7)),
        Token::Control(":".to_string(), Location(3, 8)),
        Token::Space(" ".to_string()),
        Token::Word("calc".to_string(), Location(3, 10), Location(3, 13)),
        Token::Brackets("(1px;)".to_string(), Location(3, 14), Location(3, 19)),
        Token::Space("\n  ".to_string()),
        Token::Control("}".to_string(), Location(4, 3)),
        Token::Space("\n".to_string()),
        Token::Comment("/* small screen */".to_string(), Location(5, 1), Location(5, 18)),
        Token::Space("\n".to_string()),
        Token::AtWord("@media".to_string(), Location(6, 1), Location(6, 6)),
        Token::Space(" ".to_string()),
        Token::Word("screen".to_string(), Location(6, 8), Location(6, 13)),
        Token::Space(" ".to_string()),
        Token::Control("{".to_string(), Location(6, 15)),
        Token::Control("}".to_string(), Location(6, 16)),
    ]);
}

#[test]
#[ignore]
fn it_panics_on_unclosed_string() {
    // test_panic(" \"".to_string(), ":1:2: Unclosed quote");
}

#[test]
#[ignore]
fn it_panics_on_unclosed_comment() {
    // test_panic(" /*".to_string(), ":1:2: Unclosed comment");
}

#[test]
#[ignore]
fn it_panics_on_unclosed_url() {
    // test_panic(" url(".to_string(), ":1:4: Unclosed bracket");
}
