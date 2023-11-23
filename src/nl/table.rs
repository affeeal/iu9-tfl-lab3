use std::collections::HashMap;
use std::collections::HashSet;

use crate::config::{ALPHABET, EPSILON};
use crate::mat::Mat;

pub struct Table<'a> {
    mat: &'a dyn Mat,
    prefixes: HashSet<String>,
    suffixes: HashSet<String>,
    data: HashMap<String, HashMap<String, bool>>,
}

impl<'a> Table<'a> {
    pub fn new(mat: &'a dyn Mat) -> Self {
        Self {
            mat,
            prefixes: HashSet::from([EPSILON.to_string()]),
            suffixes: HashSet::from([EPSILON.to_string()]),
            data: HashMap::from([(
                EPSILON.to_string(),
                HashMap::from([(EPSILON.to_string(), mat.check_membership(EPSILON))]),
            )]),
        }
    }

    pub fn insert_prefix(&mut self, prefix: &str) {
        if self.prefixes.contains(prefix) {
            return;
        }

        self.prefixes.insert(prefix.to_string());

        let mut suffix_to_membership = HashMap::<String, bool>::new();
        for suffix in &self.suffixes {
            let word = format!("{prefix}{suffix}");
            suffix_to_membership.insert(suffix.to_string(), self.mat.check_membership(&word));
        }

        self.data.insert(prefix.to_string(), suffix_to_membership);
    }

    pub fn insert_suffix(&mut self, suffix: &str) {
        if self.suffixes.contains(suffix) {
            return;
        }

        self.suffixes.insert(suffix.to_string());

        for (prefix, suffix_to_membership) in &mut self.data {
            let word = format!("{prefix}{suffix}");
            suffix_to_membership.insert(suffix.to_string(), self.mat.check_membership(&word));
        }
    }

    pub fn insert_prefix_with_joined_alphabet(&mut self, prefix: &str) {
        self.insert_prefix(prefix);

        for letter in ALPHABET.chars() {
            let new_prefix = format!("{prefix}{letter}");
            self.insert_prefix(&new_prefix);
        }
    }
}
