use std::collections::HashMap;

use bnf::Grammar;
use grammars::cfg
::Production;

use crate::{grammars::{GrammarMat, cfg::CFG}, mat::Mat};

pub mod automata;
pub mod config;
pub mod grammars;
pub mod mat;
pub mod nl;
pub mod pump;

fn niam() {
    use bnf::Grammar;

    let input = "<dna> ::= <base> | <base> <dna>
    <base> ::= 'A' | 'C' | 'G' | 'T'";
    let grammar: Grammar = input.parse().unwrap();

    let sentence = "GATTACA";

    let mut parse_trees = grammar.parse_input(sentence);
    match parse_trees.next() {
        Some(parse_tree) => println!("{}", parse_tree),
        _ => println!("Grammar could not parse sentence"),
    }

    let mat: GrammarMat = GrammarMat { grammar: &grammar };

    mat.check_membership("GATTACAB");
    println!("{}",mat.check_membership("GATTACA"));
}

fn main() {
    // Пример строк с правилами грамматики
    // let lines = vec!["S -> a | A | b", "A -> a"];
    // let lines = vec!["S -> A | A S", "A -> a | c | g | t"];
    let lines = vec!["S -> aXbX|aZ", "X -> aY|bY|", "Y -> X|cc", "Z -> ZX", "D -> d"];

    // Разбор правил грамматики и создание CFG
    let mut cfg = CFG::parse(lines);

    // Вывести грамматику для проверки
    // println!("{:#?}", cfg);

    cfg.to_cnf();
    // println!("{:#?}", cfg);

    // println!("{:#?}", cfg.to_prefix_grammar());
    println!("{}", cfg.to_pretty_string());
    println!("");
    let mut preffix_cfg = cfg.to_prefix_grammar();
    preffix_cfg.to_cnf();
    println!("{}", preffix_cfg.to_pretty_string());
    println!("{:#?}", preffix_cfg);
    println!("{}", preffix_cfg.to_bnf());

    let input = preffix_cfg.to_bnf();

    let grammar: Grammar = input.parse().unwrap();

    let sentence = "a";
    let mat: GrammarMat = GrammarMat { grammar: &grammar };

    mat.check_membership(sentence);

    // let mut inverted_cfg = cfg.to_inverted();
    // inverted_cfg.to_cnf();
    // println!("{}", inverted_cfg.to_pretty_string());

    // let mut suffix_cfg = cfg.to_inverted().to_prefix_grammar().to_inverted();
    // // suffix_cfg.to_cnf();
    // println!("{}", suffix_cfg.to_pretty_string());

    // let mut infix_cfg = suffix_cfg.to_prefix_grammar();
    // // infix_cfg.to_cnf();
    // println!("{}", infix_cfg.to_bnf());
    return;

    println!("{}", cfg.to_bnf());

    let input = cfg.to_bnf();

    let grammar: Grammar = input.parse().unwrap();
    let sentence = "gattaca";


    let mat: GrammarMat = GrammarMat { grammar: &grammar };

    println!("{}",mat.check_membership("gattaca"));
    println!("{:#?}",grammar.generate());
}



