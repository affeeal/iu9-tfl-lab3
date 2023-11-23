pub trait Automata {
    fn check_membership(&self, word: &str) -> bool;
}

pub struct AutomataImpl {
    // ...
}

impl Automata for AutomataImpl {
    fn check_membership(&self, word: &str) -> bool {
        todo!()
    }
}
