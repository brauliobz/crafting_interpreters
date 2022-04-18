use logos::Logos;

use crate::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'source_code> {
    pub type_: TokenType,
    pub lexeme: &'source_code str,
    pub line: u32,
}

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token(";")]
    Semicolon,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,

    // One or two character tokens
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,

    // Literals

    #[regex("[a-zA-Z_][a-zA-Z_0-9]*")]
    Identifier,
    #[regex("\"[^\"]*\"")]
    String,
    #[regex("[0-9]+(\\.[0-9]+)?")]
    Number,

    // Keywords
    #[token("and")]
    And,
    #[token("class")]
    Class,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("fun")]
    Fun,
    #[token("for")]
    For,
    #[token("if")]
    If,
    #[token("nil")]
    Nil,
    #[token("or")]
    Or,
    #[token("print")]
    Print,
    #[token("return")]
    Return,
    #[token("super")]
    Super,
    #[token("this")]
    This,
    #[token("true")]
    True,
    #[token("var")]
    Var,
    #[token("while")]
    While,

    #[regex("//[^\n]*", logos::skip)]
    Comment,
    #[regex("[ \t\r]+", logos::skip)]
    Whitespace,
    #[token("\n")]
    NewLine,

    #[error]
    Error
}

pub fn scan_tokens(src: &str) -> Result<Vec<Token>> {
    let mut lexer = TokenType::lexer(src).spanned();
    let mut tokens = Vec::new();
    let mut line = 1;

    while let Some((token_type, span)) = lexer.next() {
        match token_type {
            TokenType::NewLine => line += 1,
            TokenType::String => {
                tokens.push(Token {
                    type_: token_type,
                    lexeme: &src[span.clone()],
                    line,
                });
                line += (&src[span]).chars().filter(|c| *c == '\n').count() as u32;
            },
            TokenType::Error => {
                // TODO
            },
            _ => tokens.push(Token {
                type_: token_type,
                lexeme: &src[span],
                line,
            }),
        }
    }

    Ok(tokens)
}

impl<'source_code> Token<'source_code> {
    pub fn new(type_: TokenType, lexeme: &'source_code str, line: u32) -> Self {
        Token {
            type_,
            lexeme,
            line,
        }
    }
}
