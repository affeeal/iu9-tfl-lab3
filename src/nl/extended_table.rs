use std::collections::HashMap;
use std::collections::HashSet;

use crate::config::{ALPHABET, EPSILON};
use crate::mat::Mat;

// TODO? хранить в значениях by_prefixes не строки, а ссылки;
// TODO? использовать BTreeSet заместо HashSet для ускорения булевых операций;

pub struct ExtendedTable<'a> {
    mat: &'a dyn Mat,
    pub prefixes: HashSet<String>,
    pub suffixes: HashSet<String>,
    pub by_prefixes: HashMap<String, HashSet<String>>,
}

impl<'a> ExtendedTable<'a> {
    pub fn new(mat: &'a dyn Mat) -> Self {
        let mut table = Self {
            mat,
            prefixes: HashSet::new(),
            suffixes: HashSet::new(),
            by_prefixes: HashMap::new(),
        };

        table.insert_prefix(EPSILON);
        table.insert_suffix(EPSILON);

        table
    }

    pub fn insert_prefix(&mut self, prefix: &str) {
        for letter in ALPHABET.chars() {
            let new_prefix = format!("{prefix}{letter}");
            self.insert_prefix_impl(&new_prefix);
        }
    }

    fn insert_prefix_impl(&mut self, prefix: &str) {
        if self.prefixes.contains(prefix) {
            return;
        }
        self.prefixes.insert(prefix.to_string());

        let mut membership_suffixes = HashSet::new();
        for suffix in &self.suffixes {
            let word = format!("{prefix}{suffix}");
            if self.mat.check_membership(&word) {
                membership_suffixes.insert(suffix.to_string());
            }
        }

        self.by_prefixes
            .insert(prefix.to_string(), membership_suffixes);
    }

    pub fn insert_suffix(&mut self, suffix: &str) {
        if self.suffixes.contains(suffix) {
            return;
        }
        self.suffixes.insert(suffix.to_string());

        for (prefix, membership_suffixes) in &mut self.by_prefixes {
            let word = format!("{prefix}{suffix}");
            if self.mat.check_membership(&word) {
                membership_suffixes.insert(suffix.to_string());
            }
        }
    }
}
