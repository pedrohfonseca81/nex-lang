pub mod errors;
pub mod lexer;
pub mod parser;

use lexer::Scanner;

fn main() {
    let text = r#"5 + 5"#;

    let mut scanner = Scanner::new(text.to_string());
    scanner.scan_tokens();

    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
}
