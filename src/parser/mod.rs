pub mod scanner;
use crate::models::ParsedEmail;
use scanner::Scanner;
use std::collections::VecDeque;

fn check_escaped(scanner: &mut Scanner) -> (bool, i32) {
    // let mut scanner = s.clone();
    let mut count = 0;
    while let Some((_, '\\')) = scanner.peek_char() {
        scanner.next_char();
        count += 1;
    }
    // return whether escaped, then the number of escaped `\` characters encountered 
    (count % 2 != 0, count / 2)
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
        // `comment_level > 0` means that a comment block has already been encountered
        //    and not yet closed.
        match ch {
            // TODO: what about backslashes that aren't followed by a parenthesis?
            //    do we need to handle that as well? we'll handle it later.
            ')' => {
                let (is_escaped, backslash_count) = check_escaped(&mut scanner);
                if !is_escaped {
                    comment_level += 1;
                    if comments_raw.len() < (comment_idx + 1) {
                        comments_raw.push(VecDeque::new());
                    }
                }
                if comment_level > 1 || (comment_level == 1 && is_escaped) {
                    comments_raw[comment_idx].push_front(ch);
                }
                if comment_level > 0 {
                    for _ in 0..backslash_count {
                        comments_raw[comment_idx].push_front('\\')
                    }
                }
            }
            '(' => {
                let (is_escaped, backslash_count) = check_escaped(&mut scanner);
                if comment_level > 0 && !is_escaped {
                    comment_level -= 1;
                    if comment_level == 0 {
                        comment_idx += 1;
                    }
                }
                if comment_level > 0 {
                    comments_raw[comment_idx].push_front(ch);
                    for _ in 0..backslash_count {
                        comments_raw[comment_idx].push_front('\\')
                    }
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
        .rev()
        .map(|deque| deque.iter().collect::<String>())
        .collect();

    email.comments = comments;
    Ok(email)
}