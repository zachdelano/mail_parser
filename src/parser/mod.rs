pub mod scanner;
use crate::models::ParsedEmail;
use scanner::Scanner;
use std::collections::VecDeque;

/// The main entry point for the parsing logic.
pub fn parse_address(input: &str) -> Result<ParsedEmail, String> {
    if !input.contains('@') {
        return Err("Missing @ symbol".to_string());
    }

    let mut email = ParsedEmail {
        full_original: input.to_string(),
        ..Default::default()
    };

    // Use the logic from scanner.rs to fill the fields
    // Strategy: 
    // 1. Scan for comments/FWS at the end
    // 2. Locate the domain (up to the last @)
    // 3. Extract local part and plus-tag
    // 4. Extract display name if brackets exist
    let mut comment_level = 0;
    let mut comment_idx = 0;
    let mut scanner = Scanner::new(input);
    let mut comments_raw: Vec<VecDeque<char>> = vec![];
    while let Some((idx, ch)) = scanner.next_char() {
        println!("Char: {:?}, Index: {}, Level: {}", ch, idx, comment_level);
        match ch {
            ')' => {
                let preceded_by_backslash = scanner.peek_char()
                    .map(|(_, next_ch)| *next_ch == '\\')
                    .unwrap_or(false);
                println!("  Is escaped? {}", preceded_by_backslash);
                if !preceded_by_backslash {
                    comment_level += 1;
                    if comments_raw.len() < (comment_idx + 1) {
                        comments_raw.push(VecDeque::new());
                    }
                }
                if comment_level > 1 {
                    comments_raw[comment_idx].push_front(ch);
                }
            }
            '(' => {
                let preceded_by_backslash = scanner.peek_char()
                    .map(|(_, next_ch)| *next_ch == '\\')
                    .unwrap_or(false);
                if comment_level > 0 && !preceded_by_backslash {
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
    println!("Comments: {:?}", comments);
    email.comments = comments;
    Ok(email)
}