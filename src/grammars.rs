#![allow(dead_code)]

use bnf::Grammar;

use crate::{
    automata::Automata,
    mat::{EquivalenceCheckResult, Mat},
};

pub mod cfg;

pub struct GrammarMat<'a> {
    pub grammar: &'a Grammar,
}

impl<'a> Mat for GrammarMat<'a> {
    fn check_membership(&self, word: &str) -> bool {
        let mut tree = self.grammar.parse_input(word);
        tree.next().is_some()
    }

    fn check_equivalence(&self, automata: &dyn Automata) -> EquivalenceCheckResult {
        todo!()
    }

    fn get_alphabet(&self) -> String {
        todo!()
    }
}
