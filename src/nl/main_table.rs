use std::collections::HashMap;
use std::collections::HashSet;

use crate::config::EPSILON;
use crate::mat::Mat;

// TODO? хранить в значениях basic_prefixes, prefix_to_membership_suffixes и suffix_to_membership_prefixes не строки, а ссылки;
// TODO? использовать BTreeSet заместо HashSet для ускорения булевых операций;

pub struct MainTable<'a> {
    mat: &'a dyn Mat,
    pub prefixes: HashSet<String>,
    pub basic_prefixes: HashSet<String>,
    pub suffixes: HashSet<String>,
    pub prefix_to_membership_suffixes: HashMap<String, HashSet<String>>,
    pub suffix_to_membership_prefixes: HashMap<String, HashSet<String>>,
}

enum EquivalentBasicPrefixSearchResult {
    NotFound,
    Found(String),
}

impl<'a> MainTable<'a> {
    pub fn new(mat: &'a dyn Mat) -> Self {
        let mut table = Self {
            mat,
            prefixes: HashSet::new(),
            basic_prefixes: HashSet::new(),
            suffixes: HashSet::new(),
            prefix_to_membership_suffixes: HashMap::new(),
            suffix_to_membership_prefixes: HashMap::new(),
        };

        table.insert_prefix(EPSILON);
        table.insert_suffix(EPSILON);

        table
    }

    pub fn insert_prefix(&mut self, prefix: &str) {
        if self.prefixes.contains(prefix) {
            return;
        }
        self.prefixes.insert(prefix.to_owned());

        let mut membership_suffixes = HashSet::new();
        for (suffix, membership_prefixes) in &mut self.suffix_to_membership_prefixes {
            if self.mat.check_membership(&format!("{prefix}{suffix}")) {
                membership_suffixes.insert(suffix.to_owned());
                membership_prefixes.insert(prefix.to_owned());
            }
        }

        if let EquivalentBasicPrefixSearchResult::Found(equivalent_prefix) =
            self.find_equivalent_basic_prefix(&membership_suffixes)
        {
            if self.is_shorter(prefix, &equivalent_prefix) {
                self.basic_prefixes.remove(&equivalent_prefix);
                self.basic_prefixes.insert(prefix.to_owned());
            }
        } else if !self.is_covered(prefix, &membership_suffixes) {
            self.basic_prefixes.insert(prefix.to_owned());
            self.cleanup_basic_prefixes();
        }

        self.prefix_to_membership_suffixes
            .insert(prefix.to_owned(), membership_suffixes);
    }

    fn find_equivalent_basic_prefix(
        &self,
        desired_membership_suffixes: &HashSet<String>,
    ) -> EquivalentBasicPrefixSearchResult {
        for basic_prefix in &self.basic_prefixes {
            let membership_suffixes = self
                .prefix_to_membership_suffixes
                .get(basic_prefix)
                .unwrap();
            if desired_membership_suffixes == membership_suffixes {
                return EquivalentBasicPrefixSearchResult::Found(basic_prefix.to_owned());
            }
        }

        EquivalentBasicPrefixSearchResult::NotFound
    }

    fn is_shorter(&self, first_prefix: &str, second_prefix: &str) -> bool {
        // TODO? лексикографическое сравнение при равенстве длин
        first_prefix.len() < second_prefix.len()
    }

    pub fn is_covered(&self, prefix: &str, membership_suffixes: &HashSet<String>) -> bool {
        let non_membership_suffixes = self.suffixes.difference(membership_suffixes);

        let mut unfit_prefixes = HashSet::new();
        unfit_prefixes.insert(prefix.to_owned());

        for suffix in non_membership_suffixes {
            let membership_prefixes = self.suffix_to_membership_prefixes.get(suffix).unwrap();
            for prefix in membership_prefixes {
                unfit_prefixes.insert(prefix.to_owned());
            }
        }

        for suffix in membership_suffixes {
            let membership_prefixes = self.suffix_to_membership_prefixes.get(suffix).unwrap();
            if membership_prefixes
                .difference(&unfit_prefixes)
                .next()
                .is_none()
            {
                return false;
            }
        }

        true
    }

    fn cleanup_basic_prefixes(&mut self) {
        let mut not_basic_prefixes_anymore = Vec::new();

        for prefix in &self.basic_prefixes {
            let membership_suffixes = self.prefix_to_membership_suffixes.get(prefix).unwrap();
            if self.is_covered(prefix, membership_suffixes) {
                not_basic_prefixes_anymore.push(prefix.to_owned());
            }
        }

        for prefix in &not_basic_prefixes_anymore {
            self.basic_prefixes.remove(prefix);
        }
    }

    pub fn insert_suffix(&mut self, suffix: &str) {
        if self.suffixes.contains(suffix) {
            return;
        }
        self.suffixes.insert(suffix.to_owned());

        let mut membership_prefixes = HashSet::new();
        for (prefix, membership_suffixes) in &mut self.prefix_to_membership_suffixes {
            let word = format!("{prefix}{suffix}");
            if self.mat.check_membership(&word) {
                membership_prefixes.insert(prefix.to_owned());
                membership_suffixes.insert(suffix.to_owned());
            }
        }

        self.suffix_to_membership_prefixes
            .insert(suffix.to_owned(), membership_prefixes);
    }
}
