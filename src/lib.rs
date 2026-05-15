pub mod models;
pub mod parser;

pub use models::ParsedEmail;

/// Public API to parse an email string
pub fn parse(input: &str) -> Result<ParsedEmail, String> {
    ParsedEmail::parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let res = parse("dev@egin.tech").unwrap();
        assert_eq!(res.domain, "egin.tech");
        assert_eq!(res.local_part, "dev");
    }

    // ... Paste the rest of the TDD tests provided in the previous response here
}