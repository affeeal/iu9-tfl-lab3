use crate::{config::PUMP_TESTS, mat::Mat};

pub struct ShatteredWord {
    w1: String,
    w2: String,
    w3: String,
    w4: String,
    w5: String,
}

pub trait Pumper {
    fn pump_word(&self, word: &ShatteredWord) -> bool;
}

pub struct PumperImpl<'a> {
    mat: &'a dyn Mat,
}

impl<'a> Pumper for PumperImpl<'a> {
    fn pump_word(&self, word: &ShatteredWord) -> bool {
        for i in 0..PUMP_TESTS {
            let pumped_word: String = format!(
                "{}{}{}{}{}",
                word.w1,
                word.w2.repeat(i).to_string(),
                word.w3,
                word.w4.repeat(i).to_string(),
                word.w5
            );
            let res: bool = self.mat.check_membership(&pumped_word);
            if !res {
                return false;
            }
        }

        return true;
    }
}
