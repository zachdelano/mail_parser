// Add Default and Clone here
#[derive(Debug, Default, Clone, PartialEq)] 
pub struct ParsedEmail {
    pub display_name: Option<String>,
    pub local_part: String,
    pub domain: String,
    pub comments: Vec<String>,
    pub plus_tag: Option<String>,
    pub full_original: String,
}

impl ParsedEmail {
    pub fn parse(input: &str) -> Result<Self, String> {
        // This calls the logic we put in the parser module
        crate::parser::parse_address(input)
    }

    pub fn canonicalize(&self) -> String {
        format!("{}@{}", self.local_part, self.domain)
    }
}