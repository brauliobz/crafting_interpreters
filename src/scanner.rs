use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn scan_tokens(source_code: &str) -> Vec<Token> {
    todo!()
}

/// Returns the token, the next position to be read, and the current line
fn scan_token(src: &str, line: u32) -> Result<(Token, &str, u32)> {
    use TokenType::*;

    let mut chars = src.chars();
    let char_0 = chars.next();

    if char_0.is_none() {
        return Ok((Token::new(Eof, src, line), src, line));
    }
    let char_0 = char_0.unwrap();

    // single chars

    let single_char_token =
        |type_: TokenType| Ok((Token::new(type_, &src[0..1], line), &src[1..], line));

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
        ' ' | '\t' | '\r' => return single_char_token(Whitespace), // TODO consume more
        '\n' => {
            return Ok((
                Token::new(Whitespace, &src[0..1], line),
                &src[1..],
                line + 1,
            ))
        }
        _ => (),
    }

    // one or two chars, and comments

    let two_char_token =
        |type_: TokenType| Ok((Token::new(type_, &src[0..2], line), &src[2..], line));

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

    // string, numbers, keywords, identifiers
    match char_0 {
        '"' => return string(src, line),
        '0'..='9' => return Ok(number(src, line)),
        'a'..='z' | 'A'..='Z' | '_' => return Ok(identifier_or_keyword(src, line)),
        _ => (),
    }

    Err(Error::UnexpectedCharacter)
}

fn comment(src: &str, line: u32) -> (Token, &str, u32) {
    if let Some(pos) = src.find('\n') {
        (
            Token::new(TokenType::Comment, &src[0..(pos + 1)], line),
            &src[(pos + 1)..],
            line + 1,
        )
    } else {
        // end of file ends the comment
        (
            Token::new(TokenType::Comment, src, line),
            &src[src.len()..],
            line,
        )
    }
}

#[test]
fn test_comment() {
    let (token, after, line) = comment("// this is a comment\ntest = 10;", 10);
    assert_eq!(
        token,
        Token::new(TokenType::Comment, "// this is a comment\n", 10)
    );
    assert_eq!(after, "test = 10;");
    assert_eq!(line, 11);
}

#[test]
fn test_comment_at_line_end() {
    let (token, after, line) = comment("//\ntest = 10;", 10);
    assert_eq!(token, Token::new(TokenType::Comment, "//\n", 10));
    assert_eq!(after, "test = 10;");
    assert_eq!(line, 11);
}

#[test]
fn test_comment_end_of_file() {
    let (token, after, line) = comment("// this is a comment", 10);
    assert_eq!(
        token,
        Token::new(TokenType::Comment, "// this is a comment", 10)
    );
    assert_eq!(after, "");
    assert_eq!(line, 10);
}

fn string(src: &str, line: u32) -> Result<(Token, &str, u32)> {
    let end = src[1..].find('"');
    if let Some(end) = end {
        let end = end + 1;
        let new_lines = src[0..=end].chars().filter(|c| *c == '\n').count();
        Ok((
            Token::new(TokenType::String, &src[0..=end], line),
            &src[end + 1..],
            line + new_lines as u32,
        ))
    } else {
        Err(Error::UnterminatedString)
    }
}

#[test]
fn test_string() {
    let result = string(r#""hello, world"; x = 10;"#, 10);
    assert!(result.is_ok());
    let (token, after, line) = result.unwrap();
    assert_eq!(
        token,
        Token::new(TokenType::String, r#""hello, world""#, 10)
    );
    assert_eq!(after, "; x = 10;");
    assert_eq!(line, 10);
}

#[test]
fn test_string_not_ended() {
    let result = string(r#""hello, world; x = 10;"#, 10);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::UnterminatedString);
}

#[test]
fn test_multiline_string() {
    let result = string("\"hello\nworld\"; x = 10;", 10);
    assert!(result.is_ok());
    let (token, after, line) = result.unwrap();
    assert_eq!(
        token,
        Token::new(TokenType::String, "\"hello\nworld\"", 10)
    );
    assert_eq!(after, "; x = 10;");
    assert_eq!(line, 11);
}

fn number(src: &str, line: u32) -> (Token, &str, u32) {
    todo!()
}

fn identifier_or_keyword(src: &str, line: u32) -> (Token, &str, u32) {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'source_code> {
    type_: TokenType,
    lexeme: &'source_code str,
    line: u32,
}

#[derive(Debug, PartialEq, Eq)]
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
    Identifier,
    String,
    Number,

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
