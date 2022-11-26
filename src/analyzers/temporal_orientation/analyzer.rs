use std::collections::HashMap;
use std::ops::{Div, Mul};

use serde::{Deserialize, Serialize};

use crate::TextItem;

use super::data::WEIGHTS_TEMPORAL;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum TemporalOrientationClass {
    #[serde(rename = "PAST_OR_NOT")]
    PastOrNot,
    #[serde(rename = "PRESENT_OR_NOT")]
    PresentOrNot,
    #[serde(rename = "FUTURE_OR_NOT")]
    FutureOrNot,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemporalOrientationInterpretation {
    Likely,
    Neutral,
    Unlikely,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalOrientationAnalysis {
    pub past: TemporalOrientationInterpretation,
    pub present: TemporalOrientationInterpretation,
    pub future: TemporalOrientationInterpretation,
}

// intercepts
const INTERCEPT_PAST: f64 = -0.649406376419;
const INTERCEPT_PRESENT: f64 = 0.236749577324;
const INTERCEPT_FUTURE: f64 = -0.570547567181;

fn establish_interpretation(val: f64) -> TemporalOrientationInterpretation {
    match val {
        _ if val > 0.0 => TemporalOrientationInterpretation::Likely,
        _ if val < 0.0 => TemporalOrientationInterpretation::Unlikely,
        _ => TemporalOrientationInterpretation::Neutral,
    }
}

impl From<HashMap<TemporalOrientationClass, f64>> for TemporalOrientationAnalysis {
    fn from(value: HashMap<TemporalOrientationClass, f64>) -> Self {
        dbg!(&value);

        let past =
            value
                .get(&TemporalOrientationClass::PastOrNot)
                .unwrap_or(&0.0);

        let present =
            *value
                .get(&TemporalOrientationClass::PresentOrNot)
                .unwrap_or(&0.0);

        let future =
            *value
                .get(&TemporalOrientationClass::FutureOrNot)
                .unwrap_or(&0.0);

        TemporalOrientationAnalysis {
            past: establish_interpretation(past + INTERCEPT_PAST),
            present: establish_interpretation(present + INTERCEPT_PRESENT),
            future: establish_interpretation(future + INTERCEPT_FUTURE),
        }
    }
}

pub struct TemporalOrientation {
    pub items: HashMap<String, Vec<(TemporalOrientationClass, f64)>>,
}

impl TemporalOrientation {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        for item in WEIGHTS_TEMPORAL.iter() {
            items
                .entry(item.term.clone())
                .or_insert_with(Vec::new)
                .push((item.class, item.weight));
        }

        Self {
            items
        }
    }

    #[inline(always)]
    pub fn get_entry(&self, term: &str) -> Option<&Vec<(TemporalOrientationClass, f64)>> {
        self.items.get(term)
    }

    #[inline(always)]
    pub fn get_score(&self, item: &TextItem, term: &str, total_counts: f64) -> Option<Vec<(TemporalOrientationClass, f64)>> {
        let word_freqs =
            *item.get_term_frequency(term)? as f64;

        let total_freqs =
            item.word_count as f64;
            //total_counts;

        //dbg!(
            //word_freqs,
            //total_freqs,
        //);

        Some(
            self.get_entry(term)?
                .iter()
                .map(|(class, weight)| {
                    (
                        *class,
                        word_freqs
                            .div(total_freqs)
                            .mul(weight)
                    )
                })
                .collect(),
        )
    }

    #[inline(always)]
    pub fn run(&self, item: &TextItem) -> TemporalOrientationAnalysis {
        let matching_words =
            item
                .bigram_freqs
                .iter()
                .chain(
                    item.word_freqs
                        .iter()
                )
                .filter(|(term, _)| {
                    self.get_entry(term)
                        .is_some()
                })
                .collect::<Vec<(&String, &usize)>>();

        let match_counts =
            matching_words
                .iter()
                .map(|(_, f)| *f)
                .sum::<usize>();

        matching_words
            .iter()
            .filter_map(|(word, _)| self.get_score(item, word, match_counts as f64))
            .flatten()
            .fold(HashMap::new(), |mut acc, (class, score)| {
                acc.entry(class)
                    .and_modify(|e| *e += score)
                    .or_insert(score);

                acc
            })
            .into()
    }
}