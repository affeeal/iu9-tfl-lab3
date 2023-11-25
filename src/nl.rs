mod table;

use std::collections::HashSet;

use crate::automata::Automata;
use crate::config::ALPHABET;
use crate::mat::Mat;
use crate::nl::table::{Table, TableType};

pub trait Nl {
    fn build_automata(&mut self) -> Box<dyn Automata>;
}

pub struct NlImpl<'a> {
    mat: &'a dyn Mat,
    main_table: Table<'a>,
    extended_table: Table<'a>,
}

impl<'a> Nl for NlImpl<'a> {
    fn build_automata(&mut self) -> Box<dyn Automata> {
        todo!()
    }
}

enum CompletenessCheckResult {
    Ok,
    UncoveredPrefix(String),
}

enum ConsistencyCheckResult {
    Ok,
    DistinguishingSuffix(char),
}

impl<'a> NlImpl<'a> {
    pub fn new(mat: &'a dyn Mat) -> Self {
        Self {
            mat,
            main_table: Table::new(TableType::Main, mat),
            extended_table: Table::new(TableType::Extended, mat),
        }
    }

    fn insert_prefix(&mut self, prefix: &str) {
        for i in 1..prefix.len() {
            let new_prefix = &prefix[0..i];

            self.main_table.insert_prefix(new_prefix);
            self.extended_table.insert_prefix(new_prefix);
        }
    }

    fn insert_suffix(&mut self, suffix: &str) {
        self.main_table.insert_suffix(suffix);
        self.extended_table.insert_suffix(suffix);
    }

    fn check_completeness(&self) -> CompletenessCheckResult {
        let mut membership_suffixes_union = HashSet::new();

        // TODO: optimize union
        for membership_suffixes in self.main_table.data.values() {
            membership_suffixes_union.extend(membership_suffixes.clone());
        }

        // NOTE: heuristics
        if membership_suffixes_union.len() == self.main_table.suffixes.len() {
            return CompletenessCheckResult::Ok;
        }

        for (prefix, membership_suffixes) in &self.extended_table.data {
            if !membership_suffixes.is_subset(&membership_suffixes_union) {
                return CompletenessCheckResult::UncoveredPrefix(prefix.to_string());
            }
        }

        CompletenessCheckResult::Ok
    }

    fn check_consistency(&self) -> ConsistencyCheckResult {
        // TODO: optimize iteration
        for (prefix_1, membership_suffixes_1) in &self.main_table.data {
            for (prefix_2, mebership_suffixes_2) in &self.main_table.data {
                if !membership_suffixes_1.is_subset(mebership_suffixes_2) {
                    continue;
                }

                for letter in ALPHABET.chars() {
                    let new_prefix_1 = format!("{prefix_1}{letter}");
                    let new_prefix_2 = format!("{prefix_2}{letter}");

                    let new_membership_suffixes_1 =
                        self.extended_table.data.get(&new_prefix_1).unwrap();
                    let new_membership_suffixes_2 =
                        self.extended_table.data.get(&new_prefix_2).unwrap();

                    if !new_membership_suffixes_1.is_subset(new_membership_suffixes_2) {
                        return ConsistencyCheckResult::DistinguishingSuffix(letter);
                    }
                }
            }
        }

        ConsistencyCheckResult::Ok
    }
}
