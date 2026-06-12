pub mod scanner;
use crate::models::ParsedEmail;
use scanner::Scanner;
use std::collections::VecDeque;

fn check_escaped(s: &Scanner) -> bool {
    let mut scanner = s.clone();
    let mut count = 0;
    while let Some((_, '\\')) = scanner.peek_char() {
        scanner.next_char();
        count += 1;
    }
    count % 2 != 0
}

/// The main entry point for the parsing logic.
pub fn parse_address(input: &str) -> Result<ParsedEmail, String> {
    if !input.contains('@') {
        return Err("Missing @ symbol".to_string());
    }

    let mut email = ParsedEmail {
        full_original: input.to_string(),
        ..Default::default()
    };

    let mut comment_level = 0;
    let mut comment_idx = 0;
    let mut scanner = Scanner::new(input);
    let mut comments_raw: Vec<VecDeque<char>> = vec![];
    while let Some((_idx, ch)) = scanner.next_char() {
        match ch {
            ')' => {
                let is_escaped = check_escaped(&scanner);
                if !is_escaped {
                    comment_level += 1;
                    if comments_raw.len() < (comment_idx + 1) {
                        comments_raw.push(VecDeque::new());
                    }
                }
                if comment_level > 1 || (comment_level == 1 && is_escaped) {
                    comments_raw[comment_idx].push_front(ch);
                }
            }
            '(' => {
                let is_escaped = check_escaped(&scanner);
                if comment_level > 0 && !is_escaped {
                    comment_level -= 1;
                    if comment_level == 0 {
                        comment_idx += 1;
                    }
                }
                if comment_level > 0 {
                    comments_raw[comment_idx].push_front(ch);
                }
            }
            _ => {
                if comment_level > 0 {
                    comments_raw[comment_idx].push_front(ch);
                }
            }
        }
    }
    let comments: Vec<String> = comments_raw
        .into_iter()
        .map(|deque| deque.iter().collect::<String>())
        .collect();

    email.comments = comments;
    Ok(email)
}