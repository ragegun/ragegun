use std::collections::HashMap;
use std::ops::{Div, Mul};

use crate::TextItem;

use super::data::WEIGHTS_DISTRESS;

pub struct Distress {
    pub items: HashMap<String, f64>,
}

impl Distress {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        for item in WEIGHTS_DISTRESS.iter() {
            items.insert(item.term.clone(), item.rating);
        }

        Self {
            items
        }
    }

    #[inline(always)]
    pub fn get_entry(&self, term: &str) -> Option<&f64> {
        self.items.get(term)
    }

    #[inline(always)]
    pub fn get_score(&self, item: &TextItem, word: &str) -> Option<f64> {
        let word_coeff =
            ((*item.word_freqs.get(word)?) as f64)
                .div(item.word_count as f64)
                .mul(self.get_entry(word)?);

        Some(
            word_coeff
        )
    }

    #[inline(always)]
    pub fn run(&self, item: &TextItem) -> f64 {
        item.word_freqs
            .keys()
            .filter_map(|x| self.get_score(item, x))
            .sum::<f64>()
    }
}