use mail_parser_rs::parser::parse_address;

#[test]
fn test_comment_parsing() {
    let email = "test@example.com(comment)";
    println!("Parsing: {}", email);
    match parse_address(email) {
        Ok(parsed) => println!("Result: {:?}", parsed),
        Err(e) => println!("Error: {}", e),
    }

    let nested = "test@example.com(outer(inner))";
    println!("\nParsing: {}", nested);
    match parse_address(nested) {
        Ok(parsed) => println!("Result: {:?}", parsed),
        Err(e) => println!("Error: {}", e),
    }

    let escaped = r"test@example.com(escaped\))";
    println!("\nParsing: {}", escaped);
    match parse_address(escaped) {
        Ok(parsed) => println!("Result: {:?}", parsed),
        Err(e) => println!("Error: {}", e),
    }
}
