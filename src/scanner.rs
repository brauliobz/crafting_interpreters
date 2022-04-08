use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'source_code> {
    type_: TokenType,
    lexeme: &'source_code str,
    line: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub fn scan_tokens(source_code: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut src = source_code;
    let mut line = 1;

    loop {
        let (token, next_src, next_line) = scan_token(src, line)?;
        line = next_line;
        src = next_src;
        match token.type_ {
            TokenType::Comment | TokenType::Whitespace => continue,
            TokenType::Eof => break,
            _ => tokens.push(token),
        }
    }

    Ok(tokens)
}

/// Returns the token, the next position to be read, and the current line after the token
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
        '"' => string(src, line),
        '0'..='9' => Ok(number(src, line)),
        'a'..='z' | 'A'..='Z' | '_' => Ok(identifier_or_keyword(src, line)),
        _ => Err(Error::UnexpectedCharacter),
    }
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

fn number(src: &str, line: u32) -> (Token, &str, u32) {
    // some possibilities
    // 123<EOF>
    // 123     ;
    // 123.    ;
    // 123.456 ;

    let bytes = src.as_bytes();

    let mut end = 0;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }

    // no dot after integer part
    if end >= bytes.len() || bytes[end] != b'.' {
        return (
            Token::new(TokenType::Number, &src[0..end], line),
            &src[end..],
            line,
        );
    }

    let dot = end;
    end += 1;

    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }

    if end == dot + 1 {
        // no numbers after dot
        return (
            Token::new(TokenType::Number, &src[0..dot], line),
            &src[dot..],
            line,
        );
    } else {
        // numbers after dot
        return (
            Token::new(TokenType::Number, &src[0..end], line),
            &src[end..],
            line,
        );
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {

        // TODO use perfect hash

        let mut keywords = HashMap::new();

        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);

        keywords
    };
}

fn identifier_or_keyword(src: &str, line: u32) -> (Token, &str, u32) {
    // we assume that the first char is alpha or _
    let end = src
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .map(|i| i - 1)
        .unwrap_or(src.len() - 1);

    let identifier = &src[..=end];

    if let Some(keyword) = KEYWORDS.get(identifier) {
        (
            Token::new(*keyword, identifier, line),
            &src[end + 1..],
            line,
        )
    } else {
        (
            Token::new(TokenType::Identifier, identifier, line),
            &src[end + 1..],
            line,
        )
    }
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

#[cfg(test)]
mod tests {

    use crate::scanner::*;
    use std::collections::HashMap;

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
        assert_eq!(token, Token::new(TokenType::String, "\"hello\nworld\"", 10));
        assert_eq!(after, "; x = 10;");
        assert_eq!(line, 11);
    }

    #[test]
    fn test_unicode_string() {
        let result = string(r#""hello, Bráulio"; x = 10;"#, 10);
        assert!(result.is_ok());
        let (token, after, line) = result.unwrap();
        assert_eq!(
            token,
            Token::new(TokenType::String, r#""hello, Bráulio""#, 10)
        );
        assert_eq!(after, "; x = 10;");
        assert_eq!(line, 10);
    }

    #[test]
    fn test_number_integer() {
        let (token, after, line) = number("123; test = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Number, "123", 10));
        assert_eq!(after, "; test = 10;");
        assert_eq!(line, 10);
    }

    #[test]
    fn test_number_float() {
        let (token, after, line) = number("123.321; test = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Number, "123.321", 10));
        assert_eq!(after, "; test = 10;");
        assert_eq!(line, 10);
    }

    #[test]
    fn test_number_integer_dot() {
        let (token, after, line) = number("123.; test = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Number, "123", 10));
        assert_eq!(after, ".; test = 10;");
        assert_eq!(line, 10);
    }

    #[test]
    fn test_keywords() {
        let mut keywords = HashMap::new();

        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);

        for (keyword, token_type) in keywords {
            let src = keyword.to_owned() + "; test = 10;";
            assert_eq!(
                identifier_or_keyword(&src, 10),
                (Token::new(token_type, keyword, 10), "; test = 10;", 10)
            );
        }
    }

    #[test]
    fn test_identifier_start_with_letter() {
        let (token, rest, line) = identifier_or_keyword("identifier = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Identifier, "identifier", 10));
        assert_eq!(line, 10);
        assert_eq!(rest, " = 10;");
    }

    #[test]
    fn test_identifier_start_with_underscore() {
        let (token, rest, line) = identifier_or_keyword("__identifier = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Identifier, "__identifier", 10));
        assert_eq!(line, 10);
        assert_eq!(rest, " = 10;");
    }

    #[test]
    fn test_identifier_start_with_uppercase() {
        let (token, rest, line) = identifier_or_keyword("MyIdentifier = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Identifier, "MyIdentifier", 10));
        assert_eq!(line, 10);
        assert_eq!(rest, " = 10;");
    }

    #[test]
    fn test_identifier_with_number() {
        let (token, rest, line) = identifier_or_keyword("identifier20 = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Identifier, "identifier20", 10));
        assert_eq!(line, 10);
        assert_eq!(rest, " = 10;");
    }

    #[test]
    fn test_identifier_with_keyword_prefix() {
        let (token, rest, line) = identifier_or_keyword("class_ = 10;", 10);
        assert_eq!(token, Token::new(TokenType::Identifier, "class_", 10));
        assert_eq!(line, 10);
        assert_eq!(rest, " = 10;");
    }
}
