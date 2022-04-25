use rlox::scanner::*;

#[test]
fn test_scan_tokens_hello_world() {
    let src = r##"print "Hello, World";"##;
    let result = scan_tokens(src);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![
            Token::new(TokenType::Print, "print", 1),
            Token::new(TokenType::String, "\"Hello, World\"", 1),
            Token::new(TokenType::Semicolon, ";", 1),
        ]
    );
}

#[test]
fn test_scan_tokens_newline() {
    let src = r##"print
        "Hello, World";"##;
    let result = scan_tokens(src);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![
            Token::new(TokenType::Print, "print", 1),
            Token::new(TokenType::String, "\"Hello, World\"", 2),
            Token::new(TokenType::Semicolon, ";", 2),
        ]
    );
}

#[test]
fn test_scan_tokens_newline_bug() {
    let src = r##"1 + 1"##;
    let result = scan_tokens(src);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![
            Token::new(TokenType::NumberLiteral, "1", 1),
            Token::new(TokenType::Plus, "+", 1),
            Token::new(TokenType::NumberLiteral, "1", 1),
        ]
    );
}

#[test]
fn test_all_tokens() {
    let src = r#"
        (){},.+-;/*
        ! != = == > >= < <=
        identifier "string" 12345.123
        and class else false fun for if nil or print return super this true var while
        
        // comment
        "#;

    let result = scan_tokens(src);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![
            Token::new(TokenType::LeftParen, "(", 2),
            Token::new(TokenType::RightParen, ")", 2),
            Token::new(TokenType::LeftBrace, "{", 2),
            Token::new(TokenType::RightBrace, "}", 2),
            Token::new(TokenType::Comma, ",", 2),
            Token::new(TokenType::Dot, ".", 2),
            Token::new(TokenType::Plus, "+", 2),
            Token::new(TokenType::Minus, "-", 2),
            Token::new(TokenType::Semicolon, ";", 2),
            Token::new(TokenType::Slash, "/", 2),
            Token::new(TokenType::Star, "*", 2),
            Token::new(TokenType::Bang, "!", 3),
            Token::new(TokenType::BangEqual, "!=", 3),
            Token::new(TokenType::Equal, "=", 3),
            Token::new(TokenType::EqualEqual, "==", 3),
            Token::new(TokenType::Greater, ">", 3),
            Token::new(TokenType::GreaterEqual, ">=", 3),
            Token::new(TokenType::Less, "<", 3),
            Token::new(TokenType::LessEqual, "<=", 3),
            Token::new(TokenType::Identifier, "identifier", 4),
            Token::new(TokenType::String, "\"string\"", 4),
            Token::new(TokenType::NumberLiteral, "12345.123", 4),
            Token::new(TokenType::And, "and", 5),
            Token::new(TokenType::Class, "class", 5),
            Token::new(TokenType::Else, "else", 5),
            Token::new(TokenType::False, "false", 5),
            Token::new(TokenType::Fun, "fun", 5),
            Token::new(TokenType::For, "for", 5),
            Token::new(TokenType::If, "if", 5),
            Token::new(TokenType::Nil, "nil", 5),
            Token::new(TokenType::Or, "or", 5),
            Token::new(TokenType::Print, "print", 5),
            Token::new(TokenType::Return, "return", 5),
            Token::new(TokenType::Super, "super", 5),
            Token::new(TokenType::This, "this", 5),
            Token::new(TokenType::True, "true", 5),
            Token::new(TokenType::Var, "var", 5),
            Token::new(TokenType::While, "while", 5),
        ]
    );
}
