/// Represents the state of the scanner as it moves through the string.
#[derive(Clone)]
pub struct Scanner<'a> {
    input: &'a str,
    cursor: std::iter::Peekable<std::iter::Enumerate<std::iter::Rev<std::str::Chars<'a>>>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: input.chars().rev().enumerate().peekable(),
        }
    }

    /// Advances the cursor and returns the next character and its index.
    pub fn next_char(&mut self) -> Option<(usize, char)> {
        self.cursor.next()
    }

    /// Looks at the next character without advancing the cursor.
    pub fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.cursor.peek()
    }

    // Suggested helper:
    // pub fn scan_until_at(&mut self) -> String { ... }
}
