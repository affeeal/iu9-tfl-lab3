use std::collections::HashMap;
use std::collections::HashSet;

use crate::config::{ALPHABET, EPSILON};
use crate::mat::Mat;

#[derive(PartialEq)]
pub enum TableType {
    Main,
    Extended,
}

pub struct Table<'a> {
    mat: &'a dyn Mat,
    table_type: TableType,
    pub prefixes: HashSet<String>,
    pub suffixes: HashSet<String>,
    pub data: HashMap<String, HashSet<String>>,
}

impl<'a> Table<'a> {
    pub fn new(table_type: TableType, mat: &'a dyn Mat) -> Self {
        let mut table = Self {
            mat,
            table_type,
            prefixes: HashSet::new(),
            suffixes: HashSet::new(),
            data: HashMap::new(),
        };

        table.insert_prefix(EPSILON);
        table.insert_suffix(EPSILON);

        table
    }

    pub fn insert_prefix(&mut self, prefix: &str) {
        self.insert_prefix_impl(prefix);

        if self.table_type == TableType::Extended {
            for letter in ALPHABET.chars() {
                let new_prefix = format!("{prefix}{letter}");
                self.insert_prefix_impl(&new_prefix);
            }
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
}
