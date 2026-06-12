use mail_parser_rs::parser::parse_address;

#[test]
fn test_backslash_retention() {
    // In this case, \\ should be treated as an escaped backslash,
    // so one backslash should remain, and the ')' is NOT escaped.
    let email = r"test@example.com(back\\slash)";
    let parsed = parse_address(email).unwrap();
    // The current implementation probably returns "backslash" or something else
    // because it consumes the backslashes in check_escaped.
    // Also the current implementation uses push_front and reverse iteration,
    // let's see what it actually produces.
    assert_eq!(parsed.comments, vec![r"back\\slash"]);
}

#[test]
fn test_escaped_parenthesis() {
    let email = r"test@example.com(escaped\))";
    let parsed = parse_address(email).unwrap();
    assert_eq!(parsed.comments, vec![r"escaped\)"]);
}
