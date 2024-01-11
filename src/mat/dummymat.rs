use crate::automata::Automata;

use super::{EquivalenceCheckResult, Mat};

pub struct DummyMat {
    // ...
}

impl Mat for DummyMat {
    fn check_membership(&self, _word: &str) -> bool {
        return true;
    }

    fn check_equivalence(&self, _automata: &dyn Automata) -> EquivalenceCheckResult {
        todo!()
    }
}
