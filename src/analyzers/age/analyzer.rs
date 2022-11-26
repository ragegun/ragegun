use std::collections::HashMap;
use std::ops::{Div, Mul};

use crate::TextItem;

use super::data::WEIGHTS_AGE;

pub struct Age {
    pub items: HashMap<String, f32>,
    pub intercept: f32,
}

impl Age {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        for item in WEIGHTS_AGE.iter() {
            items.insert(item.term.clone(), item.weight);
        }

        let intercept = *items.get("_intercept").expect("Cannot find intercept value!");

        Self {
            items,
            intercept,
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
            + self.intercept
    }
}