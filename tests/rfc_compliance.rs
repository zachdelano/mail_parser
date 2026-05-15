use mail_parser_rs::ParsedEmail;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_address() {
        let input = "dev@egin.tech";
        let parsed = ParsedEmail::parse(input).unwrap();
        assert_eq!(parsed.local_part, "dev");
        assert_eq!(parsed.domain, "egin.tech");
    }

    #[test]
    fn test_display_name_and_brackets() {
        let input = "\"Engineer @ Egin\" <dev@egin.tech>";
        let parsed = ParsedEmail::parse(input).unwrap();
        assert_eq!(parsed.display_name, Some("Engineer @ Egin".to_string()));
        assert_eq!(parsed.local_part, "dev");
        assert_eq!(parsed.domain, "egin.tech");
    }

    #[test]
    fn test_nested_comments_and_fws() {
        // CFWS should be ignored or extracted
        let input = "dev@egin.tech (Primary (Work @ Office))";
        let parsed = ParsedEmail::parse(input).unwrap();
        assert_eq!(parsed.domain, "egin.tech");
        assert!(parsed.comments.contains(&"Primary (Work @ Office)".to_string()));
    }

    #[test]
    fn test_plus_addressing() {
        let input = "dev+updates@egin.tech";
        let parsed = ParsedEmail::parse(input).unwrap();
        assert_eq!(parsed.local_part, "dev");
        assert_eq!(parsed.plus_tag, Some("updates".to_string()));
    }

    #[test]
    fn test_quoted_local_part_with_at() {
        let input = "\"user@name\"@egin.tech";
        let parsed = ParsedEmail::parse(input).unwrap();
        assert_eq!(parsed.local_part, "\"user@name\"");
        assert_eq!(parsed.domain, "egin.tech");
    }

    #[test]
    fn test_canonicalization_and_duplicates() {
        let addr1 = ParsedEmail::parse("dev @ egin . tech").unwrap();
        let addr2 = ParsedEmail::parse("DEV@EGIN.TECH").unwrap();
        
        // Both should result in the same unique string
        assert_eq!(addr1.canonicalize(), "dev@egin.tech");
        assert_eq!(addr2.canonicalize(), "dev@egin.tech");
    }

    #[test]
    fn test_invalid_no_at() {
        let input = "egin.tech";
        assert!(ParsedEmail::parse(input).is_err());
    }
}