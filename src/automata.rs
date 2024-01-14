#![allow(dead_code)]

pub mod reachability;
pub mod str_generator;

use std::any::Any;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::config::EPSILON;

pub const START: usize = 0;

pub trait Automata {
    fn as_any(&self) -> &dyn Any;

    fn check_membership(&self, word: &str) -> bool;

    fn determinize(&self) -> Box<dyn Automata>;

    fn get_complement(&self) -> Box<dyn Automata>;

    fn intersect(&self, other: &dyn Automata) -> Box<dyn Automata>;

    fn generate_word(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub struct AutomataImpl {
    pub size: usize,
    pub transitions: Vec<Vec<HashSet<String>>>,
    pub start_states: Vec<bool>,
    pub finite_states: Vec<bool>,
}

#[derive(Eq, Hash, PartialEq)]
struct IntersectionState {
    first_state: usize,
    label: String,
    second_state: usize,
}

impl IntersectionState {
    fn get_start() -> Self {
        Self {
            first_state: START,
            label: DUMMY_LABEL.to_string(),
            second_state: START,
        }
    }
}

struct IntersectionDetails {
    state: usize,
    is_finite: bool,
    incoming_states: Vec<IntersectionState>,
}

const DUMMY_LABEL: &str = "";
const DUMMY_STATE: usize = 0;

impl Automata for AutomataImpl {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn check_membership(&self, word: &str) -> bool {
        todo!()
    }

    fn determinize(&self) -> Box<dyn Automata> {
        #[derive(Eq, Hash, PartialEq)]
        struct Transition {
            from: usize,
            label: String,
            to: usize,
        }

        let start_subset = BTreeSet::from([START]);
        let mut subset_to_state = HashMap::from([(start_subset.to_owned(), START)]);
        let mut state_to_subset = HashMap::from([(START, start_subset)]);
        let mut state_counter = START + 1;
        let mut states_to_visit = VecDeque::from([START]);

        let mut finite_states = HashSet::<usize>::new();
        let mut transitions = HashSet::<Transition>::new();

        while let Some(state) = states_to_visit.pop_front() {
            let mut label_to_subset = HashMap::<String, BTreeSet<usize>>::new();
            let closure = self.get_epsilon_closure(state_to_subset.get(&state).unwrap());

            for closure_state in closure {
                if self.finite_states[closure_state] {
                    finite_states.insert(state);
                }

                for (next_state, labels) in self.transitions[closure_state].iter().enumerate() {
                    for label in labels {
                        if label.eq(&EPSILON) {
                            continue;
                        }

                        if let Some(next_subset) = label_to_subset.get_mut(label) {
                            next_subset.insert(next_state);
                        } else {
                            label_to_subset.insert(label.to_owned(), BTreeSet::from([next_state]));
                        }
                    }
                }
            }

            for (label, next_subset) in &label_to_subset {
                let next_state: usize;

                if let Some(state) = subset_to_state.get(next_subset) {
                    next_state = *state;
                } else {
                    next_state = state_counter;
                    state_counter += 1;

                    subset_to_state.insert(next_subset.to_owned(), next_state);
                    state_to_subset.insert(next_state, next_subset.to_owned());
                    states_to_visit.push_back(next_state);
                }

                let transition = Transition {
                    from: state,
                    label: label.to_owned(),
                    to: next_state,
                };
                transitions.insert(transition);
            }
        }

        let mut automata = Self::new(state_counter);

        for transition in transitions {
            automata.transitions[transition.from][transition.to]
                .insert(transition.label.to_owned());
        }

        for state in finite_states {
            automata.finite_states[state] = true;
        }

        Box::new(automata)
    }

    fn get_complement(&self) -> Box<dyn Automata> {
        todo!() // доработать существующую версию для новой версии AutomataImpl
    }

    fn intersect(&self, other: &dyn Automata) -> Box<dyn Automata> {
        let other = other.as_any().downcast_ref::<AutomataImpl>().unwrap();

        let mut state_to_details = HashMap::<IntersectionState, IntersectionDetails>::new();
        state_to_details.insert(
            IntersectionState::get_start(),
            IntersectionDetails {
                state: START,
                is_finite: self.finite_states[START] && other.finite_states[START],
                incoming_states: Vec::<IntersectionState>::new(),
            },
        );

        todo!()
    }

    fn generate_word(&self) -> String {
        todo!();
    }
}

impl AutomataImpl {
    pub fn new(size: usize) -> Self {
        let mut start_states = vec![false; size];
        start_states[START] = true;

        let transitions = vec![vec![HashSet::<String>::new(); size]; size];

        let finite_states = vec![false; size];

        Self {
            start_states,
            transitions,
            finite_states,
            size,
        }
    }

    fn get_epsilon_closure(&self, subset: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut closure = BTreeSet::<usize>::new();

        for state in subset {
            closure.append(&mut self.get_state_epsilon_closure(*state));
        }

        closure
    }

    fn get_state_epsilon_closure(&self, state: usize) -> BTreeSet<usize> {
        let mut visited_states = BTreeSet::<usize>::new();
        let mut states_to_visit = VecDeque::from([state]);

        while let Some(state) = states_to_visit.pop_front() {
            visited_states.insert(state);

            for (next_state, labels) in self.transitions[state].iter().enumerate() {
                for label in labels {
                    if label.eq(&EPSILON) && !visited_states.contains(&next_state) {
                        states_to_visit.push_back(next_state);
                    }
                }
            }
        }

        visited_states
    }

    pub fn is_start_state(&self, i: usize) -> bool {
        return self.start_states[i];
    }

    pub fn is_finite_state(&self, i: usize) -> bool {
        return self.finite_states[i];
    }

    pub fn is_empty(&self) -> bool {
        self.size == 1
            && self.is_start_state(START)
            && self.transitions[START][START].is_empty()
            && !self.is_finite_state(START)
    }
}
