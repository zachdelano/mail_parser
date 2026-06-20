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
    // return whether escaped, then the number of escaped `\` characters encountered.
    // Note: count / 2 is correct and compliant with RFC 5322 because the escaping
    // backslash (e.g. in \)) is purely syntactic and must be stripped from the comment value.
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
    let mut domain_raw: VecDeque<char> = VecDeque::new();
    let mut local_raw: VecDeque<char> = VecDeque::new();
    let mut encountered_at_sign = false;
    while let Some((_idx, ch)) = scanner.next_char() {
        // `comment_level > 0` means that a comment block has already been encountered
        //    and not yet closed.
        match ch {
            // Note: Backslashes escaping any other character (e.g., \a) also act
            // as escapes and are stripped as per RFC 5322 quoted-pair rules.
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
            '\\' => {
                let (is_escaped, backslash_count) = check_escaped(&mut scanner);
                if comment_level > 0 {
                    if is_escaped {
                        comments_raw[comment_idx].push_front(ch);
                    }
                    for _ in 0..backslash_count {
                        comments_raw[comment_idx].push_front('\\')
                    }
                }
            }
            '@' => {
                let (is_escaped, backslash_count) = check_escaped(&mut scanner);
                if !is_escaped && comment_level == 0 && !encountered_at_sign {
                    encountered_at_sign = true
                }
                for _ in 0..backslash_count {
                    comments_raw[comment_idx].push_front('\\')
                }
            }
            _ => {
                if comment_level > 0 {
                    comments_raw[comment_idx].push_front(ch);
                } else if encountered_at_sign {
                    local_raw.push_front(ch);
                } else {
                    domain_raw.push_front(ch);
                }
            }
        }
    }
    let comments: Vec<String> = comments_raw
        .into_iter()
        .rev()
        .map(|deque| deque.iter().collect::<String>())
        .collect();

    // Note: Comments are correctly stored unescaped in accordance with RFC 5322.

    let domain: String = domain_raw.into_iter().collect();
    let local: String = local_raw.into_iter().collect();
    email.comments = comments;
    email.domain = domain;
    email.local_part = local;
    Ok(email)
}