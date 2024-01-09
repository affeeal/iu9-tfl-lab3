#![allow(dead_code)]

pub const START_STATE: usize = 0;

pub trait Automata {
    fn check_membership(&self, word: &str) -> bool;

    fn determinize(&self) -> Box<dyn Automata>;

    fn intersect(&self, other: &dyn Automata) -> Box<dyn Automata>;

    // Получить дополнение для ДКА. Для НКА результат неопределён.
    fn get_complement(&self) -> Box<dyn Automata>;
}

pub struct AutomataImpl {
    pub size: usize,
    pub transitions: Vec<Vec<Option<String>>>,
    pub start_states: Vec<bool>,
    pub finite_states: Vec<bool>,
}

impl Automata for AutomataImpl {
    fn check_membership(&self, word: &str) -> bool {
        todo!()
    }

    fn determinize(&self) -> Box<dyn Automata> {
        todo!()
    }

    fn intersect(&self, other: &dyn Automata) -> Box<dyn Automata> {
        todo!()
    }

    fn get_complement(&self) -> Box<dyn Automata> {
        todo!()
    }
}

impl AutomataImpl {
    pub fn new(size: usize) -> Self {
        let mut start_states = vec![false; size];
        start_states[START_STATE] = true;

        let transition_matrix = vec![vec![None; size]; size];

        let finite_states = vec![false; size];

        Self {
            start_states,
            transitions: transition_matrix,
            finite_states,
            size,
        }
    }
}
