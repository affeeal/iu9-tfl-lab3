use std::collections::{BTreeSet, HashMap};

type Symbol = String;
type Terminal = String;
type NonTerminal = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Production {
    pub elements: Vec<Symbol>,
}

impl Production {
    fn is_epsilon(&self) -> bool {
        self.elements.is_empty()
    }
}

#[derive(Debug)]
pub struct CFG {
    pub non_terminals: BTreeSet<NonTerminal>,
    pub terminals: BTreeSet<Terminal>,
    pub productions: HashMap<NonTerminal, Vec<Production>>,
    pub start_symbol: NonTerminal,
}


// TODO: buy one more hqd and split this impl into several impls
impl CFG {
    pub fn new(
        non_terminals: BTreeSet<NonTerminal>,
        terminals: BTreeSet<Terminal>,
        productions: HashMap<NonTerminal, Vec<Production>>,
        start_symbol: NonTerminal,
    ) -> CFG {
        CFG {
            non_terminals,
            terminals,
            productions,
            start_symbol,
        }
    }

    fn add_production(&mut self, non_terminal: NonTerminal, production: Production) {
        self.productions
            .entry(non_terminal)
            .or_insert(Vec::new())
            .push(production);
    }

    pub fn parse(lines: Vec<&str>) -> CFG {
        let mut non_terminals = BTreeSet::new();
        let mut terminals = BTreeSet::new();
        let mut productions = HashMap::new();
        let mut start_symbol = String::new();

        for line in lines {
            let parts: Vec<&str> = line.split("->").collect();
            if parts.len() != 2 {
                eprintln!("Неверный формат строки: {}", line);
                continue;
            }
            let lhs = parts[0].trim().to_string(); // Левая часть продукции (нетерминал)
            let rhs = parts[1].trim(); // Правая часть продукции

            if start_symbol.is_empty() {
                start_symbol = lhs.clone();
            }

            non_terminals.insert(lhs.clone());

            let production_parts: Vec<&str> = rhs.split('|').collect();
            for prod_part in production_parts {
                let mut elements: Vec<Symbol> = Vec::new();

                for ch in prod_part.trim().chars() {
                    let sym = ch.to_string();
                    if ch.is_uppercase() {
                        // Если символ в верхнем регистре, добавляем как нетерминал
                        if !non_terminals.contains(&sym) {
                            non_terminals.insert(sym.clone());
                        }
                        elements.push(sym);
                    } else if ch.is_lowercase() {
                        // Если символ в нижнем регистре, добавляем как терминал
                        if !terminals.contains(&sym) {
                            terminals.insert(sym.clone());
                        }
                        elements.push(sym);
                    }
                }

                productions
                    .entry(lhs.clone())
                    .or_insert_with(Vec::new)
                    .push(Production { elements });
            }
        }

        CFG::new(non_terminals, terminals, productions, start_symbol)
    }

    pub fn to_bnf(&self) -> String {
        let mut bnf_representation = String::new();

        // Отдельно обрабатываем стартовый символ
        if let Some(start_productions) = self.productions.get(&self.start_symbol) {
            bnf_representation.push_str(&self.format_productions_to_bnf(&self.start_symbol, start_productions));
        }

        // Получаем все нетерминалы, кроме стартового, и сортируем их алфавитно
        let mut non_terminal_list: Vec<_> = self.non_terminals.iter().filter(|nt| **nt != self.start_symbol).collect();
        non_terminal_list.sort_unstable();

        // Формируем строки правил для остальных нетерминалов в алфавитном порядке
        for nt in non_terminal_list {
            if let Some(rhs_list) = self.productions.get(nt) {
                bnf_representation.push_str(&self.format_productions_to_bnf(nt, rhs_list));
            }
        }

        bnf_representation
    }

    // Вспомогательная функция для форматирования продукций в строку БНФ
    fn format_productions_to_bnf(&self, nt: &NonTerminal, rhs_list: &Vec<Production>) -> String {
        let mut rhs_strings: Vec<String> = rhs_list.iter().map(|rhs| {
            rhs.elements.iter().map(|symbol| {
                if self.non_terminals.contains(symbol) {
                    format!("<{}>", symbol)
                } else {
                    format!("'{}'", symbol)
                }
            }).collect::<Vec<_>>().join(" ")
        }).collect();

        // Сортируем список правил для нетерминала
        rhs_strings.sort_unstable();
        format!("<{}> ::= {}\n", nt, rhs_strings.join(" | "))
    }

    pub fn to_pretty_string(&self) -> String {
        let mut result = String::new();

        // Функция для объединения продукций одного нетерминала в одну строку
        let prod_to_string = |prods: &Vec<Production>| -> String {
            prods.iter()
                .map(|prod| prod.elements.join(" "))
                .collect::<Vec<_>>()
                .join(" | ")
        };

        // Сначала добавляем правила для начального символа грамматики
        if let Some(prods) = self.productions.get(&self.start_symbol) {
            let prods_string = prod_to_string(prods);
            result.push_str(&format!("{} -> {}\n", self.start_symbol, prods_string));
        }

        // Получаем ключи для нетерминалов и сортируем их, исключая начальный символ
        let mut non_terminals: Vec<_> = self.non_terminals.iter().collect();
        non_terminals.sort_unstable();
        non_terminals.retain(|nt| **nt != self.start_symbol);

        // Добавляем правила для остальных нетерминалов в алфавитном порядке
        for nt in non_terminals {
            if let Some(prods) = self.productions.get(nt) {
                let prods_string = prod_to_string(prods);
                result.push_str(&format!("{} -> {}\n", nt, prods_string));
            }
        }

        result
    }

    pub fn to_cnf(&mut self) {
        // step 1
        self.eliminate_long_rules();

        // step 2
        self.remove_epsilon_rules();

        // step 3
        self.remove_chain_rules();

        // step 4
        // let productive_non_terminals = self.find_productive_non_terminals();
        // println!("{:#?}", productive_non_terminals);
        self.eliminate_unproductive_rules();
        // let reachable_non_terminals = self.find_reachable_non_terminals2();
        // println!("{:#?}", reachable_non_terminals);
        self.remove_rules_with_unreachable_symbols();

        // step 5
        self.replace_terminals_with_non_terminals();
        // self.add_new_start_symbol();
    }

    // Вспомогательная функция для генерации нового уникального нетерминала
    fn next_non_terminal(&mut self) -> NonTerminal {
        let new_nt = format!("S{}", self.non_terminals.len());
        self.non_terminals.insert(new_nt.clone());
        new_nt
    }

    fn eliminate_long_rules(&mut self) {
        let mut new_productions = HashMap::new();

        for (nt, prods) in self.productions.clone().iter() {
            let mut transformed_prods = Vec::new();

            for prod in prods {
                // Делим правило, если оно длиннее двух символов
                if prod.elements.len() > 2 {
                    let mut elements = prod.elements.clone();
                    let mut current_nt = nt.clone();
                    while elements.len() > 2 {
                        // Создаём новые промежуточные правила
                        let first_elem = elements.remove(0);
                        let new_nt = self.next_non_terminal();
                        let new_prod_leftover = elements.clone();

                        new_productions
                            .entry(current_nt)
                            .or_insert_with(Vec::new)
                            .push(Production {
                                elements: vec![first_elem, new_nt.clone()],
                            });

                        current_nt = new_nt;
                        elements = new_prod_leftover;
                    }
                    new_productions
                        .entry(current_nt)
                        .or_insert_with(Vec::new)
                        .push(Production { elements });
                } else {
                    // Если продукция уже соответствует формату, просто добавляем её
                    transformed_prods.push(prod.clone());
                }
            }
            new_productions
                .entry(nt.clone())
                .or_insert(Vec::<Production>::new())
                .extend(transformed_prods);
        }

        self.productions = new_productions;
    }

    fn remove_epsilon_rules(&mut self) {
        let mut eps_non_terminals: Vec<NonTerminal> = Vec::new();

        for (nt, productions) in &self.productions {
            if productions.iter().any(|p| p.is_epsilon()) {
                eps_non_terminals.push(nt.clone());
            }
        }

        let mut new_productions = HashMap::new();

        for (nt, productions) in &self.productions {
            let mut new_production_set = Vec::new();

            for prod in productions {
                if !prod.is_epsilon() || nt == &self.start_symbol {
                    new_production_set.push(prod.clone());
                }

                for eps_nt in &eps_non_terminals {
                    let mut new_elements = prod.elements.clone();
                    if let Some(pos) = new_elements.iter().position(|e| e == eps_nt) {
                        new_elements.remove(pos);
                        if !new_elements.is_empty() || nt == &self.start_symbol {
                            new_production_set.push(Production {
                                elements: new_elements,
                            });
                        }
                    }
                }
            }
            new_productions.insert(nt.clone(), new_production_set);
        }

        self.productions = new_productions;

        eps_non_terminals.retain(|nt| self.productions.get(nt).unwrap_or(&vec![]).is_empty());
        self.non_terminals
            .retain(|nt| !eps_non_terminals.contains(nt));
    }

    fn remove_chain_rules(&mut self) {
        let mut new_productions = HashMap::new();
        let mut chain_rules: HashMap<NonTerminal, Vec<NonTerminal>> = HashMap::new();

        // Идентификация и регистрация цепных правил
        for (nt, prods) in &self.productions {
            let production_symbols: Vec<NonTerminal> = prods
                .iter()
                .filter_map(|p| {
                    if p.elements.len() == 1 && self.non_terminals.contains(&p.elements[0]) {
                        Some(p.elements[0].clone())
                    } else {
                        None
                    }
                })
                .collect();

            chain_rules.insert(nt.clone(), production_symbols);
        }

        // Вычисление транзитивных замыканий для каждого нетерминала
        for nt in self.non_terminals.clone() {
            let mut closure = chain_rules.get(&nt).cloned().unwrap_or_default();
            let mut index = 0;
            while index < closure.len() {
                if let Some(next_rules) = chain_rules.get(&closure[index]) {
                    for next_rule in next_rules {
                        if !closure.contains(next_rule) {
                            closure.push(next_rule.clone());
                        }
                    }
                }
                index += 1;
            }
            chain_rules.insert(nt, closure);
        }

        // Создание продукций без цепных правил
        for (nt, prods) in &self.productions {
            let mut prod_set: Vec<Production> = Vec::new();
            for prod in prods {
                if prod.elements.len() != 1 || !self.non_terminals.contains(&prod.elements[0]) {
                    // Если это не цепная продукция, добавляем её
                    prod_set.push(prod.clone());
                }
            }

            if let Some(closure) = chain_rules.get(nt) {
                for closure_nt in closure {
                    if let Some(closure_prods) = self.productions.get(closure_nt) {
                        for closure_prod in closure_prods {
                            if closure_prod.elements.len() != 1
                                || !self.non_terminals.contains(&closure_prod.elements[0])
                            {
                                prod_set.push(closure_prod.clone());
                            }
                        }
                    }
                }
            }

            new_productions.insert(nt.clone(), prod_set);
        }

        self.productions = new_productions;

        for prods in self.productions.values_mut() {
            prods.sort_by(|a, b| a.elements.cmp(&b.elements));
            prods.dedup();
        }
    }

    fn find_productive_non_terminals(&self) -> BTreeSet<NonTerminal> {
        let mut productive: BTreeSet<NonTerminal> = BTreeSet::new();
        let mut changed = true;

        while changed {
            changed = false;
            for (nt, prods) in &self.productions {
                if !productive.contains(nt) {
                    for prod in prods {
                        if prod
                            .elements
                            .iter()
                            .all(|s| self.terminals.contains(s) || productive.contains(s))
                        {
                            productive.insert(nt.clone());
                            changed = true;
                            break;
                        }
                    }
                }
            }
        }
        productive
    }

    // Функция для определения достижимых нетерминалов
    fn find_reachable_non_terminals(&self) -> BTreeSet<NonTerminal> {
        let mut reachable: BTreeSet<NonTerminal> = BTreeSet::new();
        let mut to_visit: Vec<NonTerminal> = vec![self.start_symbol.clone()];

        while let Some(nt) = to_visit.pop() {
            if !reachable.contains(&nt) {
                reachable.insert(nt.clone());
                if let Some(prods) = self.productions.get(&nt) {
                    for prod in prods {
                        for symbol in &prod.elements {
                            if !reachable.contains(symbol) && self.non_terminals.contains(symbol) {
                                to_visit.push(symbol.clone());
                            }
                        }
                    }
                }
            }
        }
        reachable
    }

    fn find_reachable_non_terminals2(&self) -> BTreeSet<NonTerminal> {
        let mut reachable: BTreeSet<NonTerminal> = BTreeSet::new(); // Шаг 0
        reachable.insert(self.start_symbol.clone());

        let mut changed = true;
        while changed { // Шаг 2
            changed = false;
            for nt in reachable.clone() { // Используем клон, чтобы избежать изменения коллекции во время итерации
                if let Some(prods) = self.productions.get(&nt) {
                    for prod in prods { // Шаг 1
                        for symbol in &prod.elements {
                            if self.non_terminals.contains(symbol) && reachable.insert(symbol.clone()) {
                                changed = true; // Изменилось множество достижимых нетерминалов
                            }
                        }
                    }
                }
            }
        }

        reachable
    }

    fn eliminate_unproductive_rules(&mut self) {
        let productive_non_terminals = self.find_productive_non_terminals();

         // Удаляем все продукции содержащие непорождающие нетерминальные символы
         for prods in self.productions.values_mut() {
            prods.retain(|prod| {
                prod.elements.iter().all(|symbol| self.terminals.contains(symbol) || productive_non_terminals.contains(symbol))
            });
        }

        // Удаляем пустые продукции, т.е. те, которые больше не выводят никаких нетерминалов
        self.productions.retain(|_nt, prods| !prods.is_empty());


        self.non_terminals.retain(|nt| productive_non_terminals.contains(nt));
    }

    // Функция для удаления правил с недостижимыми нетерминалами
    fn remove_rules_with_unreachable_symbols(&mut self) {
        let reachable = self.find_reachable_non_terminals();

        self.productions.retain(|nt, _| reachable.contains(nt));
        self.non_terminals.retain(|nt| reachable.contains(nt));
    }

    fn replace_terminals_with_non_terminals(&mut self) {
        let mut new_productions: HashMap<NonTerminal, Vec<Production>> = HashMap::new();

        // Вспомогательная функция для создания уникального имени для нового нетерминала
        fn make_unique_non_terminal(terminal: &Terminal) -> NonTerminal {
            let new_nt = format!("G{}", terminal);
            new_nt
        }

        // Обход всех продукций и замена терминалов на нетерминалы там, где это необходимо
        for (nt, prods) in &self.productions {
            let mut new_prod_set: Vec<Production> = Vec::new();

            for prod in prods {
                let mut new_prod: Production = Production{
                    elements: Vec::new(),
                };

                for symbol in &prod.elements {
                    if self.terminals.contains(symbol) && prod.elements.len() > 1 {
                        // Создаем новый нетерминал и добавляем правило Ui -> ui
                        let new_nt = make_unique_non_terminal(symbol);
                        self.non_terminals.insert(new_nt.clone());
                        let new_rule = Production{elements: vec![symbol.clone()]};
                        if !new_productions.entry(new_nt.clone()).or_insert_with(Vec::new).contains(&new_rule) {
                            new_productions.entry(new_nt.clone()).or_insert_with(Vec::new).push(new_rule);
                        }
                        new_prod.elements.push(new_nt);
                    } else {
                        new_prod.elements.push(symbol.clone());
                    }
                }

                new_prod_set.push(new_prod);
            }

            new_productions.insert(nt.clone(), new_prod_set);
        }

        self.productions = new_productions;
    }

    // TODO: do i need it?
    fn add_new_start_symbol(&mut self) {
        let original_start_symbol = self.start_symbol.clone();
        let new_start_symbol = format!("S{}", self.non_terminals.len() + 1);

        // Для каждого нетерминала проверяем, не используется ли оригинальный стартовый символ в правых частях
        let mut start_symbol_used = false;
        for productions in self.productions.values() {
            for prod in productions {
                if prod.elements.contains(&original_start_symbol) {
                    start_symbol_used = true;
                    break;
                }
            }
            if start_symbol_used {
                break;
            }
        }

        // Если стартовый символ используется, добавляем новое стартовое правило
        if start_symbol_used {
            // Обновляем список нетерминалов
            self.non_terminals.insert(new_start_symbol.clone());
            // Вставляем новую продукцию с новым стартовым символом, ведущим к оригинальному стартовому символу
            self.productions.insert(
                new_start_symbol.clone(),
                vec![Production {
                    elements: vec![original_start_symbol],
                }],
            );
            // Обновляем стартовый символ грамматики
            self.start_symbol = new_start_symbol;
        }
    }

    pub fn to_prefix_grammar(&self) -> CFG {
        let mut prefix_cfg = CFG {
            non_terminals: self.non_terminals.clone(),
            terminals: self.terminals.clone(),
            productions: self.productions.clone(), // Все правила будут добавлены заново
            start_symbol: format!("{}ε", self.start_symbol),
        };

        // Добавляем "ε" версии нетерминалов и генерируем новые правила для префиксов
        for nt in &self.non_terminals {
            let nt_epsilon = format!("{}ε", nt);
            prefix_cfg.non_terminals.insert(nt_epsilon.clone());

            if let Some(prod_list) = self.productions.get(nt) {
                for prod in prod_list {
                    if prod.elements.len() == 1 {
                        // В случае A -> a добавляем Aε -> a | ε
                        let prod_epsilon = vec![prod.elements[0].clone()];
                        prefix_cfg.productions.entry(nt_epsilon.clone())
                            .or_insert_with(Vec::new)
                            .push(Production { elements: prod_epsilon });
                        prefix_cfg.productions.entry(nt_epsilon.clone())
                            .or_insert_with(Vec::new)
                            .push(Production { elements: vec![]});
                    } else if prod.elements.len() == 2 {
                        // В случае A -> BC добавляем Aε -> BCε | Bε
                        let prod_epsilon = vec![prod.elements[0].clone(), format!("{}ε", prod.elements[1])];
                        prefix_cfg.productions.entry(nt_epsilon.clone())
                            .or_insert_with(Vec::new)
                            .push(Production { elements: prod_epsilon });

                        let single_prod_epsilon = vec![format!("{}ε", prod.elements[0])];
                        prefix_cfg.productions.entry(nt_epsilon.clone())
                            .or_insert_with(Vec::new)
                            .push(Production { elements: single_prod_epsilon });
                    }
                }
            }
        }

        // Добавляем стартовое правило Sε -> ε
        prefix_cfg.productions.entry(prefix_cfg.start_symbol.clone())
            .or_insert_with(Vec::new)
            .push(Production { elements: vec![]});

        prefix_cfg
    }

    pub fn to_inverted(&self) -> CFG {
        // Новая CFG с копированием терминалов и начальным символом из исходной грамматики
        let mut inverted_cfg = CFG {
            non_terminals: self.non_terminals.clone(),
            terminals: self.terminals.clone(),
            productions: HashMap::new(),
            start_symbol: self.start_symbol.clone(),
        };

        for (nt, prods) in &self.productions {
            for prod in prods {
                let mut new_prod = prod.clone();
                // Если правило является бинарным (X -> YZ), инвертируем порядок
                if prod.elements.len() == 2 {
                    new_prod.elements.swap(0, 1);
                }
                // Добавляем правило в новую грамматику, сохраняя правила A -> a и S -> ε без изменений
                inverted_cfg.productions.entry(nt.clone()).or_default().push(new_prod);
            }
        }

        inverted_cfg
    }
}
