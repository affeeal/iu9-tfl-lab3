use std::collections::HashMap;
use std::collections::HashSet;

use crate::config::{ALPHABET, EPSILON};
use crate::mat::Mat;

pub struct Table<'a> {
    mat: &'a dyn Mat,
    pub prefixes: HashSet<String>,
    pub suffixes: HashSet<String>,
    pub data: HashMap<String, HashSet<String>>,
}

impl<'a> Table<'a> {
    pub fn new(mat: &'a dyn Mat) -> Self {
        let mut membership_suffixes = HashSet::<String>::new();
        if mat.check_membership(EPSILON) {
            membership_suffixes.insert(EPSILON.to_string());
        }

        Self {
            mat,
            prefixes: HashSet::from([EPSILON.to_string()]),
            suffixes: HashSet::from([EPSILON.to_string()]),
            data: HashMap::from([(EPSILON.to_string(), membership_suffixes)]),
        }
    }

    pub fn insert_prefix(&mut self, prefix: &str) {
        if self.prefixes.contains(prefix) {
            return;
        }

        self.prefixes.insert(prefix.to_string());

        let mut membership_suffixes = HashSet::<String>::new();
        for suffix in &self.suffixes {
            let word = format!("{prefix}{suffix}");
            if self.mat.check_membership(&word) {
                membership_suffixes.insert(suffix.to_string());
            }
        }

        self.data.insert(prefix.to_string(), membership_suffixes);
    }

    pub fn insert_suffix(&mut self, suffix: &str) {
        if self.suffixes.contains(suffix) {
            return;
        }

        self.suffixes.insert(suffix.to_string());

        for (prefix, membership_suffixes) in &mut self.data {
            let word = format!("{prefix}{suffix}");
            if self.mat.check_membership(&word) {
                membership_suffixes.insert(suffix.to_string());
            }
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
