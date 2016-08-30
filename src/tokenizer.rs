extern crate regex;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Location(pub usize, pub usize);

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Word(String, Location, Location),
    AtWord(String, Location, Location),
    String(String, Location, Location),
    Comment(String, Location, Location),
    Control(String, Location),
    Brackets(String, Location, Location),
    Space(String),
    LeftParen(Location),
    RightParen(Location),
}

#[derive(Debug)]
pub struct Tokenizer {
    buf: String,
    row: usize,
    col: usize,
    pos: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new (buf: &str) -> Tokenizer {
        Tokenizer {
            buf: buf.to_string(),
            row: 1,
            col: 1,
            pos: 0,
            tokens: vec![],
        }
    }

    pub fn get_tokens (&mut self) -> &Vec<Token> {
        loop {
            match self.next() {
                Some(x) => self.tokens.push(x),
                None => { break }
            }
        }
        return &self.tokens;
    }

    fn lex_whitespace (&mut self) -> Option<Token> {
        // let TAB = '\t';
        let cr = '\r';
        let nl = '\n';

        let (start, end) = match (regex::Regex::new(r"^\s*").unwrap()).find(&self.buf[self.pos..]) {
            Some((s, e)) => (s + self.pos, e + self.pos),
            _ => return None,
        };

        self.pos = end;
        let matched = self.buf[start..end].to_string();

        for c in matched.chars() {
            if c == cr || c == nl {
                self.row = self.row + 1;
                self.col = 1;
            } else {
                self.col = self.col + 1;
            }
        }

        Some(Token::Space(matched))
    }

    fn lex_control (&mut self) -> Option<Token> {
        self.col += 1;
        self.pos += 1;
        Some(Token::Control(
            self.buf[self.pos - 1..self.pos].to_string(),
            Location(self.row, self.col - 1)
        ))
    }

    fn lex_letters (&mut self) -> Option<Token> {
        let offset = 1;
        let word_end = r"^[!]?\w*[^! ]";
        // const RE_WORD_END = /[ \n\t\r\f\(\)\{\}:;@!'"\\]|\/(?=\*)/g;jk
        let sl = Location(self.row, self.col);
        let (start, end) = match (regex::Regex::new(word_end).unwrap()).find(&self.buf[self.pos..]) {
            Some((s, e)) => (s + self.pos, e + self.pos),
            _ => {
                println!("\n\nWARNING: lex_letters matched nothing\n");
                return None
            },
        };
        let advanced = end - start;
        self.pos += advanced;
        /*
        println!("\nLexing in\n  {}", self.buf);
        println!("  \"{}\" advances column({}) by {}", self.buf[start..end].to_string(), self.col, advanced);
        println!("  ({}, {})", start, end);
        */
        self.col = self.col + advanced - offset;
        Some(Token::Word(
            self.buf[start..end].to_string(),
            sl,
            Location(self.row, self.col)
        ))
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.pos >= self.buf.len() {
            return None;
        }
        match self.buf.chars().nth(self.pos).unwrap() {
            '{' => self.lex_control(),
            ':' => self.lex_control(),
            ';' => self.lex_control(),
            '}' => self.lex_control(),
            '!' => {
                self.col += 1;
                self.lex_letters()
            },
            x if x.is_whitespace() => self.lex_whitespace(),
            x if x.is_alphanumeric() => self.lex_letters(),
            _ => None,
        }
    }
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
        pos = pos + 1;
        if c.is_whitespace() {
            if c == CR || c == NL {
                line = line + 1;
            } else {
                offset = offset + 1;
            }
            let mut ts = vec![c];
            while let Some(ch) = iter.next() {
                pos = pos + 1;
                ts.push(ch);
            }
            tokens.push(Token::Space(ts.into_iter().collect()));
        } else if c.is_alphanumeric() {
            let mut ts = vec![c];
            let start = Location(line, pos);
            while let Some(ch) = iter.next() {
                pos = pos + 1;
                ts.push(ch);
            }
            tokens.push(Token::Word(
                    ts.into_iter().collect(),
                    start,
                    Location(line, pos)
                ));
        }
    }
    tokens
}

