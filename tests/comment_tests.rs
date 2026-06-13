use mail_parser_rs::parser::parse_address;

#[test]
fn test_basic_comments() {
    let inputs = vec![
        ("test@example.com (comment)", vec!["comment"]),
        ("test@example.com (one) (two)", vec!["one", "two"]),
        ("(front) test@example.com", vec!["front"]),
        ("test (middle) @example.com", vec!["middle"]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).unwrap();
        assert_eq!(result.comments, expected, "Failed on input: {}", input);
    }
}

#[test]
fn test_nested_comments() {
    let inputs = vec![
        ("test@example.com (outer (inner))", vec!["outer (inner)"]),
        ("test@example.com (level1 (level2 (level3)))", vec!["level1 (level2 (level3))"]),
        ("test@example.com (a (b) c (d) e)", vec!["a (b) c (d) e"]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).unwrap();
        assert_eq!(result.comments, expected, "Failed on input: {}", input);
    }
}

#[test]
fn test_escaped_comments() {
    let inputs = vec![
        (r"test@example.com (escaped 1 \))", vec!["escaped 1 )"]),
        (r"test@example.com (escaped 2 \()", vec!["escaped 2 ("]),
        (r"test@example.com (slash \\)", vec![r"slash \"]),
        (r"test@example.com (\\ \( \))", vec![r"\ ( )"]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).expect(&format!("Failed to parse: {}", input));
        assert_eq!(result.comments, expected, "Failed on input: {}", input);
    }
}

#[test]
fn test_semantic_boundaries() {
    // Comments should NOT be parsed inside quoted strings or domain literals
    let inputs: Vec<(&str, Vec<String>)> = vec![
        ("\"user(not a comment)\"@example.com", vec![]),
        ("user@[127.0.0.1(not a comment)]", vec![]),
        (r"test\@example.com \(not a comment\)", vec![]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).unwrap();
        assert_eq!(result.comments, expected, "Failed on input: {}", input);
    }
}

#[test]
fn test_comment_edge_cases() {
    let inputs = vec![
        ("test@example.com ()", vec![""]),
        ("test@example.com (  spaced  )", vec!["  spaced  "]),
        ("test@example.com (folded\r\n comment)", vec!["folded\r\n comment"]),
    ];

    for (input, expected) in inputs {
        let result = parse_address(input).unwrap();
        assert_eq!(result.comments, expected, "Failed on input: {}", input);
    }
}
