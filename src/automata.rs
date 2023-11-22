use crate::config::Config;

pub trait Automata {
    fn check_belonging(&self, word: String) -> bool;
}

struct AutomataImpl {
    config: Config,
    // ...
}

impl Automata for AutomataImpl {
    fn check_belonging(&self, word: String) -> bool {
        todo!()
    }
}
