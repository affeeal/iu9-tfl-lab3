use std::collections::VecDeque;

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use super::{reachability::Reachability, AutomataImpl};

pub struct StringGenerator<'a> {
    automata: &'a AutomataImpl,
    reachability: Reachability,
    rng: ThreadRng,
}

impl<'a> StringGenerator<'a> {
    const FINITE_STATE_PROBABILITY: f64 = 0.25;
    const COMPLETE_WORD_PROBABILITY: f64 = 0.5;

    const EPSILON_CHAIN: [usize; 2] = [super::START; 2];
    const EPSILON_WORDS: [String; 1] = [String::new(); 1];

    const MUTATIONS_COUNT: usize = 6;

    pub fn from_automata(automata: &'a AutomataImpl) -> Self {
        Self {
            automata,
            reachability: Reachability::from_automata(automata),
            rng: rand::thread_rng(),
        }
    }

    pub fn gen_strs(&mut self, count: usize) -> Vec<String> {
        // Empty automata corner case
        if self.automata.is_empty() {
            return Vec::new();
        }

        let mut strings = Vec::<String>::new();

        for _ in 0..count {
            let states = self.gen_states_chain();
            let mut words = self.gen_words_chain(&states);

            strings.push(self.join_words(&words));
        }

        strings
    }

    fn gen_states_chain(&mut self) -> Vec<usize> {
        let mut states = Vec::<usize>::new();
        states.push(super::START);

        let mut current_state = super::START;
        loop {
            // Nothing to visit or can exit
            if self.reachability.as_outcoming[current_state].is_empty()
                || self.automata.is_finite_state(current_state)
                    && self.rng.gen_bool(Self::FINITE_STATE_PROBABILITY)
            {
                break;
            }

            let next_state = self.reachability.as_outcoming[current_state]
                .choose(&mut self.rng)
                .unwrap();
            states.push(next_state.clone());
            current_state = next_state.clone();
        }

        // Epsilon corner case
        if current_state.eq(&super::START) {
            return Vec::from(Self::EPSILON_CHAIN);
        }

        states
    }

    fn gen_words_chain(&mut self, states_chain: &Vec<usize>) -> Vec<String> {
        if states_chain.eq(&Self::EPSILON_CHAIN) {
            return Self::EPSILON_WORDS.to_vec();
        }

        let mut words_chain = Vec::<String>::with_capacity(states_chain.len() - 1);

        let mut first_iter = states_chain.iter();
        let mut second_iter = states_chain.iter().skip(1);

        while let Some(to) = second_iter.next() {
            let from = first_iter.next().unwrap();

            words_chain.push(self.gen_word(&from, &to));
            // NOTE: можно генерировать несколько слов на отрезке.
        }

        words_chain
    }

    fn gen_word(&mut self, from: &usize, to: &usize) -> String {
        let mut states_deq = VecDeque::<(String, usize)>::new();
        states_deq.push_back((String::new(), from.clone()));

        while let Some((word_prefix, state)) = states_deq.pop_front() {
            let mut outcoming = Vec::<(String, usize)>::new();
            for (i, letter_opt) in self.automata.transitions[state].iter().enumerate() {
                if !self.reachability.as_incoming[*to].contains(&i) && to.ne(&i) {
                    continue;
                }

                if let Some(letter) = letter_opt {
                    outcoming.push((word_prefix.clone() + &letter.to_string(), i));
                }
            }

            if outcoming.is_empty()
                || state.eq(to)
                    && self.rng.gen_bool(Self::COMPLETE_WORD_PROBABILITY)
                    && !word_prefix.is_empty()
            {
                return word_prefix;
            }

            states_deq.extend(outcoming);
        }

        unreachable!()
    }

    fn join_words(&self, words: &Vec<String>) -> String {
        let mut result = String::new();

        for word in words.iter() {
            result.push_str(word);
        }

        result
    }
}
