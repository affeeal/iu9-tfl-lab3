use std::collections::HashSet;

pub const ALPHABET: &str = "abc";
pub const EPSILON: &str = "";

pub const EQUIVALENCE_TESTS: usize = 10;
pub const REGULARITY_TESTS: usize = 10;
pub const PUMP_TESTS: usize = 10;

pub fn get_alphabet_as_hashset() -> HashSet<String> {
    let mut alphabet = HashSet::<String>::with_capacity(ALPHABET.len());

    for label in ALPHABET.chars() {
        alphabet.insert(label.to_string());
    }

    alphabet
}
