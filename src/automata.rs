pub trait Automata {
    fn check_membership(&self, word: &str) -> bool;

    fn determinize(&self) -> Box<dyn Automata>;
}

pub struct AutomataImpl {
    pub size: usize,
    pub transition_matrix: Vec<Vec<Option<char>>>,
    start_states: Vec<bool>,
    finite_states: Vec<bool>,
}

impl Automata for AutomataImpl {
    fn check_membership(&self, word: &str) -> bool {
        todo!()
    }

    fn determinize(&self) -> Box<dyn Automata> {
        todo!()
    }
}

impl AutomataImpl {
    pub fn new(size: usize) -> AutomataImpl {
        todo!()
    }

    pub fn is_start_state(state: usize) -> bool {
        todo!()
    }

    pub fn is_finite_state(&self, state: usize) -> bool {
        todo!()
    }
}
