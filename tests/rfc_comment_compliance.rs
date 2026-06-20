use mail_parser_rs::parser::parse_address;

/// RFC 5322 Section 3.2.2: Comments MUST NOT be recognized inside quoted-strings.
/// Parentheses inside quotes are literal characters.
#[test]
fn test_comments_inside_quoted_strings() {
    let inputs: Vec<(&str, Vec<&str>)> = vec![
        ("\"user(not a comment)\"@example.com", vec![]),
        ("\"(not a comment)user\"@example.com", vec![]),
        ("\"user(nested(not a comment))\"@example.com", vec![]),
        ("\"user\\(not a comment\\)\"@example.com", vec![]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).unwrap();
        assert_eq!(
            result.comments, expected,
            "Quoted string check failed for: {}", input
        );
    }
}

/// RFC 5322 Section 3.4.1: Comments MUST NOT be recognized inside domain-literals.
/// Parentheses inside square brackets are literal characters.
#[test]
fn test_comments_inside_domain_literals() {
    let inputs: Vec<(&str, Vec<&str>)> = vec![
        ("user@[127.0.0.1(not a comment)]", vec![]),
        ("user@[(not a comment)127.0.0.1]", vec![]),
        ("user@[[127.0.0.1](not a comment)]", vec![]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).unwrap();
        assert_eq!(
            result.comments, expected,
            "Domain literal check failed for: {}", input
        );
    }
}

/// RFC 5322 Section 3.2.2: Unbalanced comments are invalid comments and should either
/// not be parsed as comments, or cause a parsing error.
/// 
/// An unmatched closing parenthesis `)` should not start a comment.
/// An unmatched opening parenthesis `(` should not end a comment or be considered a comment.
#[test]
fn test_unbalanced_parentheses() {
    let inputs = vec![
        "user@example.com (unclosed comment",
        "user@example.com unopened comment)",
        "user@example.com (closed) unclosed(",
        "user@example.com (closed) unopened)",
        "user@example.com (outer (inner unclosed)",
    ];

    for input in inputs {
        let result = parse_address(input);
        // Depending on your parser design, unbalanced inputs could either:
        // 1. Return a parsing Error.
        // 2. Ignore the malformed comment blocks entirely (comments vector is empty).
        // Here, we assert that the comments vector does NOT contain the malformed fragments.
        if let Ok(parsed) = result {
            assert!(
                parsed.comments.is_empty(),
                "Malformed comments should not be extracted on input: {}. Got: {:?}",
                input, parsed.comments
            );
        }
    }
}

/// RFC 5322 Section 3.2.1: Quoted-pair / escaping backslashes inside comments.
/// A backslash `\` escapes the character following it. 
/// - Two backslashes `\\` represent a literal backslash and do NOT escape the next character.
/// - In `(hello\\)world)`, the first `\` escapes the second `\`, leaving the `)` unescaped.
///   This means the comment closes at `)`.
#[test]
fn test_escaped_backslashes_in_comments() {
    let inputs = vec![
        // (escaped \\\\ backslash) -> "escaped \\ backslash"
        (r"user@example.com (escaped \\\\ backslash)", vec![r"escaped \\ backslash"]),
        
        // (escaped \\) -> Here, the first `\` escapes the second `\`. 
        // The `)` is not escaped, closing the comment.
        (r"user@example.com (escaped \\)", vec![r"escaped \"]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).unwrap();
        assert_eq!(
            result.comments, expected,
            "Escaped backslash check failed for: {}", input
        );
    }
}

/// RFC 5322 Section 3.2.2: Comments and folding white spaces (CFWS) can be interspersed 
/// between almost any token in the address. 
/// However, comments should be extracted, and the actual address components should be clean.
#[test]
fn test_interspersed_comments() {
    let input = "(c1) dev (c2) @ (c3) egin (c4) . (c5) tech (c6)";
    let result = parse_address(input).unwrap();
    
    // All comments should be collected in the order they appear (left to right)
    assert_eq!(
        result.comments,
        vec!["c1", "c2", "c3", "c4", "c5", "c6"]
    );
}
