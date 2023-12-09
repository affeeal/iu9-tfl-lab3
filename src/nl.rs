#![allow(dead_code)]

mod extended_table;
mod main_table;

use crate::automata::Automata;
use crate::config::ALPHABET;
use crate::mat::{EquivalenceCheckResult, Mat};
use crate::nl::extended_table::ExtendedTable;
use crate::nl::main_table::MainTable;

// TODO: оптимизировать итерации в check_consistency

pub trait Nl {
    fn build_automata(&mut self) -> Box<dyn Automata>;
}

pub struct NlImpl<'a> {
    mat: &'a dyn Mat,
    main_table: MainTable<'a>,
    extended_table: ExtendedTable<'a>,
}

impl<'a> Nl for NlImpl<'a> {
    fn build_automata(&mut self) -> Box<dyn Automata> {
        loop {
            if let CompletenessCheckResult::UncoveredPrefix(prefix) = self.check_completeness() {
                self.insert_prefix(&prefix);
                continue;
            }

            if let ConsistencyCheckResult::DistinguishingSuffix(suffix) = self.check_consistency() {
                self.insert_suffix(&suffix);
                continue;
            }

            let rfsa = self.build_rfsa();
            let dfsa = rfsa.determinize();

            if let EquivalenceCheckResult::Counterexample(word) =
                self.mat.check_equivalence(dfsa.as_ref())
            {
                self.insert_prefix_recursive(&word);
                continue;
            }

            break dfsa;
        }
    }
}

enum CompletenessCheckResult {
    Ok,
    UncoveredPrefix(String),
}

enum ConsistencyCheckResult {
    Ok,
    DistinguishingSuffix(String),
}

impl<'a> NlImpl<'a> {
    pub fn new(mat: &'a dyn Mat) -> Self {
        Self {
            mat,
            main_table: MainTable::new(mat),
            extended_table: ExtendedTable::new(mat),
        }
    }

    fn insert_prefix_recursive(&mut self, prefix: &str) {
        for i in 1..prefix.len() {
            self.insert_prefix(&prefix[0..i]);
        }
    }

    fn insert_prefix(&mut self, prefix: &str) {
        self.main_table.insert_prefix(prefix);
        self.extended_table.insert_prefix(prefix);
    }

    fn insert_suffix(&mut self, suffix: &str) {
        self.main_table.insert_suffix(suffix);
        self.extended_table.insert_suffix(suffix);
    }

    fn check_completeness(&self) -> CompletenessCheckResult {
        for prefix in &self.extended_table.prefixes {
            let membership_suffixes = self
                .main_table
                .prefix_to_membership_suffixes
                .get(prefix)
                .unwrap();
            if !self.main_table.is_covered(prefix, membership_suffixes) {
                return CompletenessCheckResult::UncoveredPrefix(prefix.to_owned());
            }
        }

        CompletenessCheckResult::Ok
    }

    fn check_consistency(&self) -> ConsistencyCheckResult {
        for (prefix_1, membership_suffixes_1) in &self.main_table.prefix_to_membership_suffixes {
            for (prefix_2, mebership_suffixes_2) in &self.main_table.prefix_to_membership_suffixes {
                if !membership_suffixes_1.is_subset(mebership_suffixes_2) {
                    continue;
                }

                for letter in ALPHABET.chars() {
                    let new_prefix_1 = format!("{prefix_1}{letter}");
                    let new_prefix_2 = format!("{prefix_2}{letter}");

                    let new_membership_suffixes_1 = self
                        .extended_table
                        .prefix_to_membership_suffixes
                        .get(&new_prefix_1)
                        .unwrap();
                    let new_membership_suffixes_2 = self
                        .extended_table
                        .prefix_to_membership_suffixes
                        .get(&new_prefix_2)
                        .unwrap();

                    if let Some(suffix) = new_membership_suffixes_1
                        .difference(new_membership_suffixes_2)
                        .next()
                    {
                        let distinguishing_suffix = format!("{letter}{suffix}");
                        return ConsistencyCheckResult::DistinguishingSuffix(distinguishing_suffix);
                    }
                }
            }
        }

        ConsistencyCheckResult::Ok
    }

    fn build_rfsa(&self) -> Box<dyn Automata> {
        todo!()
    }
}
