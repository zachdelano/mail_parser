# mail_parser_rs

A Rust library dedicated to parsing email strings in accordance with [RFC 5322](https://tools.ietf.org/html/rfc5322) (Internet Message Format).

## Features

- Parses email addresses into structured components (`display_name`, `local_part`, `domain`, `comments`, `plus_tag`).
- Correctly handles RFC 5322 comments (including nested comments and escaped parenthesis/characters).
- Canonicalizes email addresses.

## Usage

Add `mail_parser_rs` to your cargo dependencies:

```toml
[dependencies]
mail_parser_rs = { path = "." }
```

### Example

```rust
use mail_parser_rs::parse;

fn main() {
    let input = "\"Engineer @ Egin\" <dev@egin.tech> (Primary)";
    match parse(input) {
        Ok(parsed) => {
            println!("Display Name: {:?}", parsed.display_name); // Some("Engineer @ Egin")
            println!("Local Part: {}", parsed.local_part);      // "dev"
            println!("Domain: {}", parsed.domain);              // "egin.tech"
            println!("Comments: {:?}", parsed.comments);        // ["Primary"]
            println!("Canonical: {}", parsed.canonicalize());   // "dev@egin.tech"
        }
        Err(e) => eprintln!("Error parsing email: {}", e),
    }
}
```

## Running Tests

Run the test suite using Cargo:

```bash
cargo test
```
