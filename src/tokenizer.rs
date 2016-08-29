
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Location(i32, i32);

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Word(String, Location, Location),
    Space(String),
    // GroupR,
}


#[allow(unused_variables, non_snake_case)]
pub fn tokenizer(input: &str) -> Vec<Token> {
    let SPACE = ' ';
    let TAB = '\t';
    let CR = '\r';
    let NL = '\n';
    let mut offset = -1;
    let mut line = 1;
    let mut pos = 0;

    let mut tokens = vec![];
    let mut iter = input.chars();

    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            if c == CR || c == NL {
                line = line + 1;
            }
            let mut ts = vec![c];
            while let Some(ch) = iter.next() {
                ts.push(ch);
            }
            tokens.push(Token::Space(ts.into_iter().collect()));
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compiles() {
        let expected:Vec<Token> = vec![];
        assert_eq!(tokenizer(""), expected);
    }

    #[test]
    fn it_tokenizes_single_character_space() {
        assert_eq!(tokenizer(" "), vec![Token::Space(" ".to_string())]);
        assert_eq!(tokenizer(&'\n'.to_string()), vec![Token::Space('\n'.to_string())]);
        assert_eq!(tokenizer(&'\r'.to_string()), vec![Token::Space('\r'.to_string())]);
        assert_eq!(tokenizer(&'\t'.to_string()), vec![Token::Space('\t'.to_string())]);
    }

    #[test]
    fn it_tokenizes_space() {
        assert_eq!(
            tokenizer("\r\n \t"),
            vec![
                Token::Space("\r\n \t".to_string())
            ]
        );
    }

    /*
    #[test]
    fn it_tokenizes_word() {
        assert_eq!(
            tokenizer("ab"),
            vec![Token::Word("ab".to_string(), Location(1, 1), Location(1, 2))]
        );
    }
    */

}

