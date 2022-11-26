use std::collections::HashMap;
use std::ops::{Div, Mul};

use serde::{Deserialize, Serialize};

use crate::TextItem;

use super::data::WEIGHTS_GENDER;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenderInterpretation {
    Male,
    Female,
    Unknown,
}

pub struct Gender {
    pub items: HashMap<String, f64>,
    pub intercept: f64,
}

impl Gender {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        for item in WEIGHTS_GENDER.iter() {
            items.insert(item.term.clone(), item.weight);
        }

        let intercept =
            *items.get("_intercept")
                .expect("Cannot find intercept value!");

        Self {
            items,
            intercept,
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
    pub fn run(&self, item: &TextItem) -> GenderInterpretation {
        match item.word_freqs
            .keys()
            .filter_map(|x| self.get_score(item, x))
            .sum::<f64>()
            + self.intercept {
            x if x < 0.0 => GenderInterpretation::Male,
            x if x > 0.0 => GenderInterpretation::Female,
            _ => GenderInterpretation::Unknown,
        }
    }
}