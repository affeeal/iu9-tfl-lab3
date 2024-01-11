#![allow(dead_code)]

use bnf::Grammar;

use crate::{mat::{Mat, EquivalenceCheckResult}, automata::Automata};

pub mod cfg;

pub struct GrammarMat<'a> {
    pub grammar: &'a Grammar,
}

impl<'a> Mat for GrammarMat<'a> {
    fn check_membership(&self, word: &str) -> bool {
        let mut tree = self.grammar.parse_input(word);
        match tree.next() {
            None => return false,
            Some(tree) => return true
        }
        return true;
    }

    fn check_equivalence(&self, automata: &dyn Automata) -> EquivalenceCheckResult {
        todo!()
    }
}