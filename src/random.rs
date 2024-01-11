pub struct StringGenerator<'a> {
    automata: &'a ndfa::Automata,
    reachability: Reachability,
    rng: ThreadRng,
}
