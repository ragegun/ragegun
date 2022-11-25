use std::collections::HashMap;
use std::ops::{Div, Mul};

use crate::assets::WEIGHTS_DISTRESS;
use crate::TextItem;

pub struct Distress {
    pub items: HashMap<String, f32>,
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
    pub fn get_entry(&self, term: &str) -> Option<&f32> {
        self.items.get(term)
    }

    #[inline(always)]
    pub fn get_score(&self, item: &TextItem, word: &str) -> Option<f32> {
        let word_coeff =
            ((*item.word_freqs.get(word)?) as f32)
                .div(item.word_count as f32)
                .mul(self.get_entry(word)?);

        Some(
            word_coeff
        )
    }

    #[inline(always)]
    pub fn run(&self, item: &TextItem) -> f32 {
        item.word_freqs
            .keys()
            .filter_map(|x| self.get_score(item, x))
            .sum::<f32>()
    }
}