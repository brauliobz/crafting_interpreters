use crate::error::Error;

pub fn scan_tokens(source_code: &str) -> Vec<Token> {
    todo!()
}

/// Returns the token, the next position to be read, and the current line
fn scan_token(src: &str, line: u32) -> Result<(Token, &str, u32), Error> {
    use TokenType::*;

    let mut chars = src.chars();
    let char_0 = chars.next();

    if let None = char_0 {
        return Ok((Token::new(Eof, src, line), src, line));
    }
    let char_0 = char_0.unwrap();

    // single chars

    let single_char_token = |type_: TokenType| {
        Ok((Token::new(type_, &src[0..1], line), &src[1..], line))
    };

    match char_0 {
        '(' => return single_char_token(LeftParen),
        ')' => return single_char_token(RightParen),
        '{' => return single_char_token(LeftBrace),
        '}' => return single_char_token(RightBrace),
        ',' => return single_char_token(Comma),
        '.' => return single_char_token(Dot),
        '-' => return single_char_token(Minus),
        '+' => return single_char_token(Plus),
        ';' => return single_char_token(Semicolon),
        '*' => return single_char_token(Star),
        _ => (),
    }

    // one or two chars, and comments

    let two_char_token = |type_: TokenType| {
        Ok((Token::new(type_, &src[0..2], line), &src[2..], line))
    };

    let char_1 = chars.next();
    match (char_0, char_1) {
        ('!', Some('=')) => return two_char_token(BangEqual),
        ('!', _) => return single_char_token(Bang),
        ('=', Some('=')) => return two_char_token(EqualEqual),
        ('=', _) => return single_char_token(Equal),
        ('<', Some('=')) => return two_char_token(LessEqual),
        ('<', _) => return single_char_token(Less),
        ('>', Some('=')) => return two_char_token(GreaterEqual),
        ('>', _) => return single_char_token(Greater),
        ('/', Some('/')) => return Ok(comment(src, line)),
        ('/', _) => return single_char_token(Slash),
        _ => (),
    }
    
    // TODO white spaces (increment line on '\n')
    // TODO string literals (increment line on '\n')
    // TODO number literals
    // TODO identifiers
    // TODO keywords and fixed literals

    Err(Error{})
}

fn comment(src: &str, line: u32) -> (Token, &str, u32) {
    todo!()
}

fn consume<'code>(
    source_code: &'code str,
    pattern: &str, /* TODO allow regex */
) -> Option<(Token<'code>, &'code str)> {
    todo!()
}

fn peek<'code>(
    source_code: &'code str,
    pattern: &str, /* TODO allow regex */
) -> Option<(Token<'code>, &'code str)> {
    todo!()
}

pub struct Token<'source_code> {
    type_: TokenType,
    lexeme: &'source_code str,
    // literal: Box<Any>, NOT NEEDED
    line: u32,
}

pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Comment,
    Whitespace,
    Eof,
}

impl<'source_code> Token<'source_code> {
    fn new(type_: TokenType, lexeme: &'source_code str, line: u32) -> Self {
        Token {
            type_,
            lexeme,
            line,
        }
    }
}
