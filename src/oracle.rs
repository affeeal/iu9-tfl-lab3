use crate::automata::Automata;
use crate::config::Config;

pub enum EquivalenceResponse {
    Ok,
    Counterexample(String),
}

pub trait Oracle {
    fn beloning_request(&self, word: &str) -> bool;

    fn equivalence_request(&self, automata: &impl Automata) -> EquivalenceResponse;
}

struct OracleImpl {
    config: Config,
    // ...
}

impl Oracle for OracleImpl {
    fn beloning_request(&self, word: &str) -> bool {
        todo!()
    }

    fn equivalence_request(&self, automata: &impl Automata) -> EquivalenceResponse {
        todo!()
    }
}
