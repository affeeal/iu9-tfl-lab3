pub struct Config {
    pub alphabet: Vec<char>,
    pub equivalence_tests: usize,
    pub regularity_tests: usize,
    pub pump_tests: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alphabet: vec!['a', 'b', 'c'],
            equivalence_tests: 10,
            regularity_tests: 10,
            pump_tests: 10,
        }
    }
}
