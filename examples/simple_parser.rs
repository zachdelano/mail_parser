use mail_parser_rs::parse;

fn main() {
    let inputs = [
        "John Doe <john.doe@example.com> (Work)",
        "John Joseph <john.joseph@example.com> (Work (Personal))",
        "John Foo <john.Foo@example.com> (Work (Personal) Plus Extra)",
        "John Bar <john.Foo@example.com> (Work (Personal) Plus Extra (And a Little More))",
        "foo.bar@example.com (Work (Personal) Plus Foo) (And a Little Bar)"
    ];
    for input in inputs {
        match parse(input) {
            Ok(email) => {
                println!("Successfully parsed: {:?}", email);
                println!("Canonical: {}", email.canonicalize());
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}