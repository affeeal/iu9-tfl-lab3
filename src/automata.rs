#![allow(dead_code)]

use std::any::Any;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::config::EPSILON;

pub const START_STATE: usize = 0;

pub trait Automata {
    fn as_any(&self) -> &dyn Any;

    fn check_membership(&self, word: &str) -> bool;

    fn determinize(&self) -> Box<dyn Automata>;

    fn get_complement(&self) -> Box<dyn Automata>;

    fn intersect(&self, other: &dyn Automata) -> Box<dyn Automata>;
}

#[derive(Debug, PartialEq)]
pub struct AutomataImpl {
    pub size: usize,
    pub transitions: Vec<Vec<Option<String>>>,
    pub start_states: Vec<bool>,
    pub finite_states: Vec<bool>,
}

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

        let start_subset = BTreeSet::from([START_STATE]);
        let mut subset_to_state = HashMap::from([(start_subset.to_owned(), START_STATE)]);
        let mut state_to_subset = HashMap::from([(START_STATE, start_subset)]);
        let mut state_counter = START_STATE + 1;
        let mut states_to_visit = VecDeque::from([START_STATE]);

        let mut finite_states = HashSet::<usize>::new();
        let mut transitions = HashSet::<Transition>::new();

        while let Some(state) = states_to_visit.pop_front() {
            let mut label_to_subset = HashMap::<String, BTreeSet<usize>>::new();
            let closure = self.get_epsilon_closure(state_to_subset.get(&state).unwrap());

            for closure_state in closure {
                if self.finite_states[closure_state] {
                    finite_states.insert(state);
                }

                for (next_state, label) in self.transitions[closure_state].iter().enumerate() {
                    if label.is_none() {
                        continue;
                    }

                    let label = label.as_ref().unwrap();
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
            automata.transitions[transition.from][transition.to] =
                Some(transition.label.to_owned());
        }

        for state in finite_states {
            automata.finite_states[state] = true;
        }

        Box::new(automata)
    }

    fn get_complement(&self) -> Box<dyn Automata> {
        todo!()
    }

    fn intersect(&self, other: &dyn Automata) -> Box<dyn Automata> {
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

            for (next_state, label) in self.transitions[state].iter().enumerate() {
                if label.is_none() {
                    continue;
                }

                let label = label.as_ref().unwrap();
                if label.eq(&EPSILON) && !visited_states.contains(&next_state) {
                    states_to_visit.push_back(next_state);
                }
            }
        }

        visited_states
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Automata, AutomataImpl};

    #[test]
    fn dfa() {
        let mut source = AutomataImpl::new(5);

        let a = Some("a".to_string());
        let b = Some("b".to_string());
        let c = Some("c".to_string());

        source.transitions = vec![
            vec![None, c.clone(), b.clone(), None, None],
            vec![None, None, b.clone(), None, None],
            vec![None, a.clone(), None, c.clone(), b.clone()],
            vec![None, None, None, None, a.clone()],
            vec![None, None, None, None, None],
        ];

        source.finite_states[1] = true;
        source.finite_states[4] = true;

        let result = source.determinize();
        // TODO: check isomorphism?
    }

    #[test]
    fn nfa() {
        let mut source = AutomataImpl::new(10);

        let a = Some("a".to_string());
        let b = Some("b".to_string());
        let epsilon = Some("".to_string());

        source.transitions = vec![
            vec![
                None,
                a.clone(),
                None,
                None,
                b.clone(),
                None,
                None,
                epsilon.clone(),
                a.clone(),
                b.clone(),
            ],
            vec![
                None,
                a.clone(),
                a.clone(),
                b.clone(),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None, None, None, None, None, None, None, None, None, None],
            vec![
                None,
                epsilon.clone(),
                None,
                b.clone(),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![
                None,
                None,
                None,
                None,
                b.clone(),
                a.clone(),
                b.clone(),
                None,
                None,
                None,
            ],
            vec![
                None,
                None,
                None,
                None,
                epsilon.clone(),
                a.clone(),
                None,
                None,
                None,
                None,
            ],
            vec![None, None, None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, None, None],
        ];

        source.finite_states = vec![
            false, false, true, false, false, false, true, true, true, true,
        ];

        let result = source.determinize();
        // TODO: check isomorphism?
    }
}
