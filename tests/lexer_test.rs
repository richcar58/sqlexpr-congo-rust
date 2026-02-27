use parser::{Lexer, TokenType, ParseError};

// ========== Helpers ==========

/// Tokenize input, return all non-EOF tokens as (type, image) pairs.
fn tokenize(input: &str) -> Vec<(TokenType, String)> {
    let mut lexer = Lexer::new(input.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token().unwrap();
        if tok.token_type == TokenType::EOF {
            break;
        }
        tokens.push((tok.token_type, tok.image.clone()));
    }
    tokens
}

/// Tokenize input, return all non-EOF tokens with offsets: (type, image, begin, end).
fn tokenize_with_offsets(input: &str) -> Vec<(TokenType, String, usize, usize)> {
    let mut lexer = Lexer::new(input.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token().unwrap();
        if tok.token_type == TokenType::EOF {
            break;
        }
        tokens.push((tok.token_type, tok.image.clone(), tok.begin_offset, tok.end_offset));
    }
    tokens
}

/// Tokenize input expecting an error. Returns the error.
fn tokenize_err(input: &str) -> ParseError {
    let mut lexer = Lexer::new(input.to_string());
    loop {
        match lexer.next_token() {
            Ok(tok) if tok.token_type == TokenType::EOF => {
                panic!("Expected tokenization error for input: {:?}", input);
            }
            Ok(_) => continue,
            Err(e) => return e,
        }
    }
}

/// Print tokens for --nocapture visibility, then assert types match expected.
fn assert_tokens(input: &str, expected: &[(TokenType, &str)]) {
    let actual = tokenize(input);
    println!("Tokens for {:?}:", input);
    for (tt, img) in &actual {
        println!("  {:?} {:?}", tt, img);
    }
    assert_eq!(
        actual.len(),
        expected.len(),
        "Token count mismatch for {:?}: got {} expected {}",
        input,
        actual.len(),
        expected.len()
    );
    for (i, ((act_type, act_img), (exp_type, exp_img))) in
        actual.iter().zip(expected.iter()).enumerate()
    {
        assert_eq!(
            act_type, exp_type,
            "Token {} type mismatch for {:?}: got {:?} expected {:?}",
            i, input, act_type, exp_type
        );
        assert_eq!(
            act_img, exp_img,
            "Token {} image mismatch for {:?}: got {:?} expected {:?}",
            i, input, act_img, exp_img
        );
    }
}

// ========== Empty / EOF ==========

#[test]
fn test_empty_input() {
    assert_tokens("", &[]);
}

#[test]
fn test_whitespace_only() {
    assert_tokens("   ", &[]);
    assert_tokens("\t\n\r", &[]);
    assert_tokens("  \t  \n  ", &[]);
}

// ========== Single-Character Operators ==========

#[test]
fn test_eq_operator() {
    assert_tokens("=", &[(TokenType::EQ, "=")]);
}

#[test]
fn test_gt_operator() {
    assert_tokens(">", &[(TokenType::GT, ">")]);
}

#[test]
fn test_lt_operator() {
    assert_tokens("<", &[(TokenType::LT, "<")]);
}

#[test]
fn test_arithmetic_operators() {
    assert_tokens("+", &[(TokenType::PLUS, "+")]);
    assert_tokens("-", &[(TokenType::MINUS, "-")]);
    assert_tokens("*", &[(TokenType::STAR, "*")]);
    assert_tokens("/", &[(TokenType::SLASH, "/")]);
    assert_tokens("%", &[(TokenType::PERCENT, "%")]);
}

#[test]
fn test_delimiters() {
    assert_tokens("(", &[(TokenType::LPAREN, "(")]);
    assert_tokens(",", &[(TokenType::COMMA, ",")]);
    assert_tokens(")", &[(TokenType::RPAREN, ")")]);
}

// ========== Multi-Character Operators ==========

#[test]
fn test_ne_operator() {
    assert_tokens("<>", &[(TokenType::NE, "<>")]);
}

#[test]
fn test_ge_operator() {
    assert_tokens(">=", &[(TokenType::GE, ">=")]);
}

#[test]
fn test_le_operator() {
    assert_tokens("<=", &[(TokenType::LE, "<=")]);
}

#[test]
fn test_multi_char_operator_disambiguation() {
    // `<>` is NE, not LT followed by GT
    assert_tokens("<>", &[(TokenType::NE, "<>")]);
    // `>=` is GE, not GT followed by EQ
    assert_tokens(">=", &[(TokenType::GE, ">=")]);
    // `<=` is LE, not LT followed by EQ
    assert_tokens("<=", &[(TokenType::LE, "<=")]);
    // `<` alone is LT when not followed by `>` or `=`
    assert_tokens("< 5", &[(TokenType::LT, "<"), (TokenType::DECIMAL_LITERAL, "5")]);
    // `>` alone is GT when not followed by `=`
    assert_tokens("> 5", &[(TokenType::GT, ">"), (TokenType::DECIMAL_LITERAL, "5")]);
}

// ========== Keywords (case-insensitive) ==========

#[test]
fn test_keywords_uppercase() {
    assert_tokens("NOT", &[(TokenType::NOT, "NOT")]);
    assert_tokens("AND", &[(TokenType::AND, "AND")]);
    assert_tokens("OR", &[(TokenType::OR, "OR")]);
    assert_tokens("BETWEEN", &[(TokenType::BETWEEN, "BETWEEN")]);
    assert_tokens("LIKE", &[(TokenType::LIKE, "LIKE")]);
    assert_tokens("ESCAPE", &[(TokenType::ESCAPE, "ESCAPE")]);
    assert_tokens("IN", &[(TokenType::IN, "IN")]);
    assert_tokens("IS", &[(TokenType::IS, "IS")]);
    assert_tokens("TRUE", &[(TokenType::TRUE, "TRUE")]);
    assert_tokens("FALSE", &[(TokenType::FALSE, "FALSE")]);
    assert_tokens("NULL", &[(TokenType::NULL, "NULL")]);
}

#[test]
fn test_keywords_lowercase() {
    assert_tokens("not", &[(TokenType::NOT, "not")]);
    assert_tokens("and", &[(TokenType::AND, "and")]);
    assert_tokens("or", &[(TokenType::OR, "or")]);
    assert_tokens("between", &[(TokenType::BETWEEN, "between")]);
    assert_tokens("like", &[(TokenType::LIKE, "like")]);
    assert_tokens("escape", &[(TokenType::ESCAPE, "escape")]);
    assert_tokens("in", &[(TokenType::IN, "in")]);
    assert_tokens("is", &[(TokenType::IS, "is")]);
    assert_tokens("true", &[(TokenType::TRUE, "true")]);
    assert_tokens("false", &[(TokenType::FALSE, "false")]);
    assert_tokens("null", &[(TokenType::NULL, "null")]);
}

#[test]
fn test_keywords_mixed_case() {
    assert_tokens("Not", &[(TokenType::NOT, "Not")]);
    assert_tokens("aNd", &[(TokenType::AND, "aNd")]);
    assert_tokens("oR", &[(TokenType::OR, "oR")]);
    assert_tokens("BeTwEeN", &[(TokenType::BETWEEN, "BeTwEeN")]);
    assert_tokens("True", &[(TokenType::TRUE, "True")]);
    assert_tokens("nUlL", &[(TokenType::NULL, "nUlL")]);
}

#[test]
fn test_keyword_preserves_original_image() {
    // The token image should be the original text, not the uppercased form
    let tokens = tokenize("not");
    assert_eq!(tokens[0].1, "not");
    let tokens = tokenize("True");
    assert_eq!(tokens[0].1, "True");
}

// ========== Identifiers ==========

#[test]
fn test_simple_identifiers() {
    assert_tokens("a", &[(TokenType::ID, "a")]);
    assert_tokens("abc", &[(TokenType::ID, "abc")]);
    assert_tokens("x1", &[(TokenType::ID, "x1")]);
    assert_tokens("foo_bar", &[(TokenType::ID, "foo_bar")]);
}

#[test]
fn test_identifier_starting_with_underscore() {
    assert_tokens("_x", &[(TokenType::ID, "_x")]);
    assert_tokens("_", &[(TokenType::ID, "_")]);
    assert_tokens("__foo", &[(TokenType::ID, "__foo")]);
    assert_tokens("_123", &[(TokenType::ID, "_123")]);
}

#[test]
fn test_keyword_prefix_is_identifier() {
    // Words that START with a keyword but are longer should be identifiers
    assert_tokens("ANDY", &[(TokenType::ID, "ANDY")]);
    assert_tokens("NOTA", &[(TokenType::ID, "NOTA")]);
    assert_tokens("ORDERING", &[(TokenType::ID, "ORDERING")]);
    assert_tokens("ISNULL", &[(TokenType::ID, "ISNULL")]);
    assert_tokens("INNER", &[(TokenType::ID, "INNER")]);
    assert_tokens("NOTIFY", &[(TokenType::ID, "NOTIFY")]);
    assert_tokens("LIKED", &[(TokenType::ID, "LIKED")]);
    assert_tokens("TRUEVALUE", &[(TokenType::ID, "TRUEVALUE")]);
    assert_tokens("NULLIFY", &[(TokenType::ID, "NULLIFY")]);
}

// ========== Numeric Literals ==========

#[test]
fn test_integer_literals() {
    assert_tokens("0", &[(TokenType::DECIMAL_LITERAL, "0")]);
    assert_tokens("1", &[(TokenType::DECIMAL_LITERAL, "1")]);
    assert_tokens("42", &[(TokenType::DECIMAL_LITERAL, "42")]);
    assert_tokens("12345", &[(TokenType::DECIMAL_LITERAL, "12345")]);
    assert_tokens("00", &[(TokenType::DECIMAL_LITERAL, "00")]);
}

#[test]
fn test_decimal_literals() {
    assert_tokens("3.14", &[(TokenType::DECIMAL_LITERAL, "3.14")]);
    assert_tokens("0.5", &[(TokenType::DECIMAL_LITERAL, "0.5")]);
    assert_tokens("100.0", &[(TokenType::DECIMAL_LITERAL, "100.0")]);
    assert_tokens("0.0", &[(TokenType::DECIMAL_LITERAL, "0.0")]);
    assert_tokens("123.456", &[(TokenType::DECIMAL_LITERAL, "123.456")]);
}

#[test]
fn test_number_dot_no_trailing_digit() {
    // `123.x`: number stops before dot (no trailing digit), dot is unexpected
    let mut lexer = Lexer::new("123.x".to_string());
    let tok1 = lexer.next_token().unwrap();
    assert_eq!(tok1.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok1.image, "123");
    let err = lexer.next_token().unwrap_err();
    println!("Error for '123.x' at dot: {}", err);
    assert!(err.message.contains("Unexpected character"));

    // `123.` at end of input: same behavior — integer then error on dot
    let mut lexer = Lexer::new("123.".to_string());
    let tok1 = lexer.next_token().unwrap();
    assert_eq!(tok1.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok1.image, "123");
    let err = lexer.next_token().unwrap_err();
    println!("Error for '123.' at dot: {}", err);
    assert!(err.message.contains("Unexpected character"));
}

// ========== String Literals ==========

#[test]
fn test_string_literals() {
    assert_tokens("'hello'", &[(TokenType::STRING_LITERAL, "'hello'")]);
    assert_tokens("'foo bar'", &[(TokenType::STRING_LITERAL, "'foo bar'")]);
    assert_tokens("'%test%'", &[(TokenType::STRING_LITERAL, "'%test%'")]);
}

#[test]
fn test_empty_string_literal() {
    assert_tokens("''", &[(TokenType::STRING_LITERAL, "''")]);
}

#[test]
fn test_string_with_special_characters() {
    assert_tokens("'hello world 123'", &[(TokenType::STRING_LITERAL, "'hello world 123'")]);
    assert_tokens("'a=b>c<d'", &[(TokenType::STRING_LITERAL, "'a=b>c<d'")]);
    assert_tokens("'(x,y)'", &[(TokenType::STRING_LITERAL, "'(x,y)'")]);
}

#[test]
fn test_adjacent_string_literals() {
    // Two strings back to back: 'ab''cd' -> STRING('ab'), STRING('cd')
    assert_tokens(
        "'ab''cd'",
        &[
            (TokenType::STRING_LITERAL, "'ab'"),
            (TokenType::STRING_LITERAL, "'cd'"),
        ],
    );
}

#[test]
fn test_string_image_includes_quotes() {
    let tokens = tokenize("'hello'");
    assert_eq!(tokens[0].1, "'hello'");
}

// ========== Whitespace Handling ==========

#[test]
fn test_whitespace_is_skipped() {
    assert_tokens(
        "a  >  5",
        &[
            (TokenType::ID, "a"),
            (TokenType::GT, ">"),
            (TokenType::DECIMAL_LITERAL, "5"),
        ],
    );
}

#[test]
fn test_leading_trailing_whitespace() {
    assert_tokens(
        "   a > 5   ",
        &[
            (TokenType::ID, "a"),
            (TokenType::GT, ">"),
            (TokenType::DECIMAL_LITERAL, "5"),
        ],
    );
}

#[test]
fn test_tabs_and_newlines_as_whitespace() {
    assert_tokens(
        "a\t>\n5",
        &[
            (TokenType::ID, "a"),
            (TokenType::GT, ">"),
            (TokenType::DECIMAL_LITERAL, "5"),
        ],
    );
}

#[test]
fn test_mixed_whitespace() {
    assert_tokens(
        " \t \n \r a",
        &[(TokenType::ID, "a")],
    );
}

// ========== Token Offsets ==========

#[test]
fn test_offsets_no_whitespace() {
    let tokens = tokenize_with_offsets("a>5");
    println!("Offsets for 'a>5': {:?}", tokens);
    assert_eq!(tokens[0], (TokenType::ID, "a".to_string(), 0, 1));
    assert_eq!(tokens[1], (TokenType::GT, ">".to_string(), 1, 2));
    assert_eq!(tokens[2], (TokenType::DECIMAL_LITERAL, "5".to_string(), 2, 3));
}

#[test]
fn test_offsets_with_whitespace() {
    // Whitespace is skipped, so offsets reflect the actual positions in the input
    let tokens = tokenize_with_offsets("a > 5");
    println!("Offsets for 'a > 5': {:?}", tokens);
    assert_eq!(tokens[0], (TokenType::ID, "a".to_string(), 0, 1));
    assert_eq!(tokens[1], (TokenType::GT, ">".to_string(), 2, 3));
    assert_eq!(tokens[2], (TokenType::DECIMAL_LITERAL, "5".to_string(), 4, 5));
}

#[test]
fn test_offsets_multi_char_tokens() {
    let tokens = tokenize_with_offsets("<> >= <=");
    println!("Offsets for '<> >= <=': {:?}", tokens);
    assert_eq!(tokens[0], (TokenType::NE, "<>".to_string(), 0, 2));
    assert_eq!(tokens[1], (TokenType::GE, ">=".to_string(), 3, 5));
    assert_eq!(tokens[2], (TokenType::LE, "<=".to_string(), 6, 8));
}

#[test]
fn test_offsets_string_literal() {
    let tokens = tokenize_with_offsets("'hello'");
    assert_eq!(tokens[0], (TokenType::STRING_LITERAL, "'hello'".to_string(), 0, 7));
}

#[test]
fn test_offsets_multi_digit_number() {
    let tokens = tokenize_with_offsets("12345");
    assert_eq!(tokens[0], (TokenType::DECIMAL_LITERAL, "12345".to_string(), 0, 5));
}

#[test]
fn test_offsets_keyword() {
    let tokens = tokenize_with_offsets("BETWEEN");
    assert_eq!(tokens[0], (TokenType::BETWEEN, "BETWEEN".to_string(), 0, 7));
}

// ========== Full Expression Tokenization ==========

#[test]
fn test_simple_comparison() {
    assert_tokens(
        "a > 5",
        &[
            (TokenType::ID, "a"),
            (TokenType::GT, ">"),
            (TokenType::DECIMAL_LITERAL, "5"),
        ],
    );
}

#[test]
fn test_equality_expression() {
    assert_tokens(
        "x = 'hello'",
        &[
            (TokenType::ID, "x"),
            (TokenType::EQ, "="),
            (TokenType::STRING_LITERAL, "'hello'"),
        ],
    );
}

#[test]
fn test_compound_expression() {
    assert_tokens(
        "a > 5 AND b = 'hello'",
        &[
            (TokenType::ID, "a"),
            (TokenType::GT, ">"),
            (TokenType::DECIMAL_LITERAL, "5"),
            (TokenType::AND, "AND"),
            (TokenType::ID, "b"),
            (TokenType::EQ, "="),
            (TokenType::STRING_LITERAL, "'hello'"),
        ],
    );
}

#[test]
fn test_is_null_expression() {
    assert_tokens(
        "x IS NULL",
        &[
            (TokenType::ID, "x"),
            (TokenType::IS, "IS"),
            (TokenType::NULL, "NULL"),
        ],
    );
}

#[test]
fn test_is_not_null_expression() {
    assert_tokens(
        "x IS NOT NULL",
        &[
            (TokenType::ID, "x"),
            (TokenType::IS, "IS"),
            (TokenType::NOT, "NOT"),
            (TokenType::NULL, "NULL"),
        ],
    );
}

#[test]
fn test_between_expression() {
    assert_tokens(
        "x BETWEEN 1 AND 10",
        &[
            (TokenType::ID, "x"),
            (TokenType::BETWEEN, "BETWEEN"),
            (TokenType::DECIMAL_LITERAL, "1"),
            (TokenType::AND, "AND"),
            (TokenType::DECIMAL_LITERAL, "10"),
        ],
    );
}

#[test]
fn test_like_expression() {
    assert_tokens(
        "name LIKE '%foo%'",
        &[
            (TokenType::ID, "name"),
            (TokenType::LIKE, "LIKE"),
            (TokenType::STRING_LITERAL, "'%foo%'"),
        ],
    );
}

#[test]
fn test_like_with_escape() {
    assert_tokens(
        "x LIKE '%a' ESCAPE '\\'",
        &[
            (TokenType::ID, "x"),
            (TokenType::LIKE, "LIKE"),
            (TokenType::STRING_LITERAL, "'%a'"),
            (TokenType::ESCAPE, "ESCAPE"),
            (TokenType::STRING_LITERAL, "'\\'"),
        ],
    );
}

#[test]
fn test_in_expression() {
    assert_tokens(
        "x IN ('a', 'b', 'c')",
        &[
            (TokenType::ID, "x"),
            (TokenType::IN, "IN"),
            (TokenType::LPAREN, "("),
            (TokenType::STRING_LITERAL, "'a'"),
            (TokenType::COMMA, ","),
            (TokenType::STRING_LITERAL, "'b'"),
            (TokenType::COMMA, ","),
            (TokenType::STRING_LITERAL, "'c'"),
            (TokenType::RPAREN, ")"),
        ],
    );
}

#[test]
fn test_arithmetic_expression() {
    assert_tokens(
        "a + b * c - d / e % f",
        &[
            (TokenType::ID, "a"),
            (TokenType::PLUS, "+"),
            (TokenType::ID, "b"),
            (TokenType::STAR, "*"),
            (TokenType::ID, "c"),
            (TokenType::MINUS, "-"),
            (TokenType::ID, "d"),
            (TokenType::SLASH, "/"),
            (TokenType::ID, "e"),
            (TokenType::PERCENT, "%"),
            (TokenType::ID, "f"),
        ],
    );
}

#[test]
fn test_parenthesized_expression() {
    assert_tokens(
        "(a OR b) AND c",
        &[
            (TokenType::LPAREN, "("),
            (TokenType::ID, "a"),
            (TokenType::OR, "OR"),
            (TokenType::ID, "b"),
            (TokenType::RPAREN, ")"),
            (TokenType::AND, "AND"),
            (TokenType::ID, "c"),
        ],
    );
}

#[test]
fn test_not_expression() {
    assert_tokens(
        "NOT x = 1",
        &[
            (TokenType::NOT, "NOT"),
            (TokenType::ID, "x"),
            (TokenType::EQ, "="),
            (TokenType::DECIMAL_LITERAL, "1"),
        ],
    );
}

#[test]
fn test_complex_expression() {
    assert_tokens(
        "a >= 1 AND b <> 'x' OR NOT c IS NULL",
        &[
            (TokenType::ID, "a"),
            (TokenType::GE, ">="),
            (TokenType::DECIMAL_LITERAL, "1"),
            (TokenType::AND, "AND"),
            (TokenType::ID, "b"),
            (TokenType::NE, "<>"),
            (TokenType::STRING_LITERAL, "'x'"),
            (TokenType::OR, "OR"),
            (TokenType::NOT, "NOT"),
            (TokenType::ID, "c"),
            (TokenType::IS, "IS"),
            (TokenType::NULL, "NULL"),
        ],
    );
}

// ========== Tokens Without Separating Whitespace ==========

#[test]
fn test_no_whitespace_between_tokens() {
    assert_tokens(
        "a>=5",
        &[
            (TokenType::ID, "a"),
            (TokenType::GE, ">="),
            (TokenType::DECIMAL_LITERAL, "5"),
        ],
    );
}

#[test]
fn test_operator_adjacent_to_string() {
    assert_tokens(
        "a='hello'",
        &[
            (TokenType::ID, "a"),
            (TokenType::EQ, "="),
            (TokenType::STRING_LITERAL, "'hello'"),
        ],
    );
}

#[test]
fn test_parentheses_adjacent() {
    assert_tokens(
        "(a)",
        &[
            (TokenType::LPAREN, "("),
            (TokenType::ID, "a"),
            (TokenType::RPAREN, ")"),
        ],
    );
}

#[test]
fn test_commas_adjacent() {
    assert_tokens(
        "('a','b')",
        &[
            (TokenType::LPAREN, "("),
            (TokenType::STRING_LITERAL, "'a'"),
            (TokenType::COMMA, ","),
            (TokenType::STRING_LITERAL, "'b'"),
            (TokenType::RPAREN, ")"),
        ],
    );
}

#[test]
fn test_unary_minus_adjacent() {
    assert_tokens(
        "-5",
        &[
            (TokenType::MINUS, "-"),
            (TokenType::DECIMAL_LITERAL, "5"),
        ],
    );
}

// ========== Boolean Literals ==========

#[test]
fn test_true_false_null_as_values() {
    assert_tokens(
        "TRUE AND FALSE OR NULL",
        &[
            (TokenType::TRUE, "TRUE"),
            (TokenType::AND, "AND"),
            (TokenType::FALSE, "FALSE"),
            (TokenType::OR, "OR"),
            (TokenType::NULL, "NULL"),
        ],
    );
}

// ========== EOF Behavior ==========

#[test]
fn test_eof_on_empty() {
    let mut lexer = Lexer::new("".to_string());
    let tok = lexer.next_token().unwrap();
    println!("EOF token: {:?}", tok);
    assert_eq!(tok.token_type, TokenType::EOF);
    assert_eq!(tok.image, "");
    assert_eq!(tok.begin_offset, 0);
    assert_eq!(tok.end_offset, 0);
}

#[test]
fn test_eof_after_tokens() {
    let mut lexer = Lexer::new("a".to_string());
    let tok1 = lexer.next_token().unwrap();
    assert_eq!(tok1.token_type, TokenType::ID);
    let tok2 = lexer.next_token().unwrap();
    assert_eq!(tok2.token_type, TokenType::EOF);
}

#[test]
fn test_repeated_eof() {
    let mut lexer = Lexer::new("".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ========== Negative Tests: Errors ==========

#[test]
fn test_error_unexpected_character_at() {
    let err = tokenize_err("@");
    println!("Error for '@': {}", err);
    assert!(err.message.contains("Unexpected character"));
}

#[test]
fn test_error_unexpected_character_hash() {
    let err = tokenize_err("#");
    println!("Error for '#': {}", err);
    assert!(err.message.contains("Unexpected character"));
}

#[test]
fn test_error_unexpected_character_bang() {
    let err = tokenize_err("!");
    println!("Error for '!': {}", err);
    assert!(err.message.contains("Unexpected character"));
}

#[test]
fn test_error_unexpected_character_tilde() {
    let err = tokenize_err("~");
    println!("Error for '~': {}", err);
    assert!(err.message.contains("Unexpected character"));
}

#[test]
fn test_error_unexpected_character_dollar() {
    let err = tokenize_err("$");
    println!("Error for '$': {}", err);
    assert!(err.message.contains("Unexpected character"));
}

#[test]
fn test_error_dot_alone() {
    let err = tokenize_err(".");
    println!("Error for '.': {}", err);
    assert!(err.message.contains("Unexpected character"));
}

#[test]
fn test_error_unexpected_after_valid_tokens() {
    // Valid tokens followed by an invalid character
    let err = tokenize_err("a > @");
    println!("Error for 'a > @': {}", err);
    assert!(err.message.contains("Unexpected character"));
}

#[test]
fn test_error_unterminated_string() {
    let err = tokenize_err("'hello");
    println!("Error for unterminated string: {}", err);
    assert!(err.message.contains("Unterminated string literal"));
}

#[test]
fn test_error_unterminated_string_with_content() {
    let err = tokenize_err("'this is not closed");
    println!("Error: {}", err);
    assert!(err.message.contains("Unterminated string literal"));
}

#[test]
fn test_error_unterminated_string_after_valid_tokens() {
    let err = tokenize_err("a = 'oops");
    println!("Error: {}", err);
    assert!(err.message.contains("Unterminated string literal"));
}

#[test]
fn test_error_unterminated_empty_string() {
    // Just a single quote with nothing after
    let err = tokenize_err("'");
    println!("Error for single quote: {}", err);
    assert!(err.message.contains("Unterminated string literal"));
}

// ========== Edge Cases ==========

#[test]
fn test_all_operators_in_sequence() {
    assert_tokens(
        "= <> > >= < <= ( , ) + - * / %",
        &[
            (TokenType::EQ, "="),
            (TokenType::NE, "<>"),
            (TokenType::GT, ">"),
            (TokenType::GE, ">="),
            (TokenType::LT, "<"),
            (TokenType::LE, "<="),
            (TokenType::LPAREN, "("),
            (TokenType::COMMA, ","),
            (TokenType::RPAREN, ")"),
            (TokenType::PLUS, "+"),
            (TokenType::MINUS, "-"),
            (TokenType::STAR, "*"),
            (TokenType::SLASH, "/"),
            (TokenType::PERCENT, "%"),
        ],
    );
}

#[test]
fn test_all_keywords_in_sequence() {
    assert_tokens(
        "NOT AND OR BETWEEN LIKE ESCAPE IN IS TRUE FALSE NULL",
        &[
            (TokenType::NOT, "NOT"),
            (TokenType::AND, "AND"),
            (TokenType::OR, "OR"),
            (TokenType::BETWEEN, "BETWEEN"),
            (TokenType::LIKE, "LIKE"),
            (TokenType::ESCAPE, "ESCAPE"),
            (TokenType::IN, "IN"),
            (TokenType::IS, "IS"),
            (TokenType::TRUE, "TRUE"),
            (TokenType::FALSE, "FALSE"),
            (TokenType::NULL, "NULL"),
        ],
    );
}

#[test]
fn test_single_char_identifier() {
    for c in 'a'..='z' {
        let s = c.to_string();
        // Skip two-letter keywords: we test single chars only
        let tokens = tokenize(&s);
        assert_eq!(tokens.len(), 1, "Failed for '{}'", s);
        assert_eq!(tokens[0].0, TokenType::ID, "Failed for '{}'", s);
    }
}

#[test]
fn test_identifier_with_digits_and_underscores() {
    assert_tokens("a1b2c3", &[(TokenType::ID, "a1b2c3")]);
    assert_tokens("foo_bar_baz", &[(TokenType::ID, "foo_bar_baz")]);
    assert_tokens("_a_1_", &[(TokenType::ID, "_a_1_")]);
}

#[test]
fn test_consecutive_operators() {
    // `<<` should be LT, LT (not a single token)
    assert_tokens("<<", &[(TokenType::LT, "<"), (TokenType::LT, "<")]);
    // `>>` should be GT, GT
    assert_tokens(">>", &[(TokenType::GT, ">"), (TokenType::GT, ">")]);
    // `==` should be EQ, EQ
    assert_tokens("==", &[(TokenType::EQ, "="), (TokenType::EQ, "=")]);
    // `+-` should be PLUS, MINUS
    assert_tokens("+-", &[(TokenType::PLUS, "+"), (TokenType::MINUS, "-")]);
}

#[test]
fn test_le_ne_adjacent() {
    // `<>=` should be NE, EQ (since `<>` matches first as NE)
    assert_tokens("<>=", &[(TokenType::NE, "<>"), (TokenType::EQ, "=")]);
}

#[test]
fn test_lt_gt_adjacent() {
    // `><` should be GT, LT
    assert_tokens("><", &[(TokenType::GT, ">"), (TokenType::LT, "<")]);
}

#[test]
fn test_number_adjacent_to_identifier() {
    // `123abc` -> number stops at 'a', then identifier
    // Actually: match_number consumes digits. Stops at 'a'. Returns "123".
    // Next call: 'a' starts an identifier. Returns "abc".
    assert_tokens(
        "123abc",
        &[
            (TokenType::DECIMAL_LITERAL, "123"),
            (TokenType::ID, "abc"),
        ],
    );
}

#[test]
fn test_string_with_newline_inside() {
    // String literals can contain newlines
    assert_tokens(
        "'line1\nline2'",
        &[(TokenType::STRING_LITERAL, "'line1\nline2'")],
    );
}

#[test]
fn test_string_with_operators_inside() {
    assert_tokens(
        "'a > b AND c'",
        &[(TokenType::STRING_LITERAL, "'a > b AND c'")],
    );
}

#[test]
fn test_large_number() {
    assert_tokens("999999999999", &[(TokenType::DECIMAL_LITERAL, "999999999999")]);
}

#[test]
fn test_decimal_with_many_places() {
    assert_tokens("3.14159265358979", &[(TokenType::DECIMAL_LITERAL, "3.14159265358979")]);
}

#[test]
fn test_error_has_location_info() {
    let err = tokenize_err("@");
    println!("Error details: line={:?} column={:?}", err.line, err.column);
    assert_eq!(err.line, Some(1));
    assert_eq!(err.column, Some(1));
}

#[test]
fn test_error_location_after_whitespace() {
    let err = tokenize_err("   @");
    println!("Error details: line={:?} column={:?}", err.line, err.column);
    assert_eq!(err.line, Some(1));
    assert_eq!(err.column, Some(4));
}

#[test]
fn test_error_location_on_second_line() {
    let err = tokenize_err("\n@");
    println!("Error details: line={:?} column={:?}", err.line, err.column);
    assert_eq!(err.line, Some(2));
    assert_eq!(err.column, Some(1));
}

#[test]
fn test_error_position_unterminated_string() {
    let err = tokenize_err("'oops");
    println!("Error details: position={:?}", err.position);
    // Position should point to the opening quote
    assert_eq!(err.position, Some(0));
}

#[test]
fn test_error_position_unterminated_string_after_whitespace() {
    let err = tokenize_err("  'oops");
    println!("Error details: position={:?}", err.position);
    assert_eq!(err.position, Some(2));
}
