mod table;

use crate::automata::Automata;
use crate::mat::Mat;
use crate::nl::table::Table;

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
            main_table: Table::new(mat),
            extended_table: Table::new(mat),
        }
    }

    fn insert_prefix(&mut self, prefix: &str) {
        for i in 1..prefix.len() {
            let new_prefix = &prefix[0..i];

            self.main_table.insert_prefix(new_prefix);
            self.extended_table
                .insert_prefix_with_joined_alphabet(new_prefix);
        }
    }

    fn check_completeness(&self) -> CompletenessCheckResult {
        todo!()
    }

    fn check_consistency(&self) -> ConsistencyCheckResult {
        todo!()
    }
}
