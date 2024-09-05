pub struct Errors {}

impl Errors {
    pub fn unexpected_character(character: char, line: i32) {
        panic!("Unexpected character '{}' at line {}", character, line);
    }
}
