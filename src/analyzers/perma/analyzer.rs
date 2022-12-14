use std::collections::HashMap;
use std::ops::{Div, Mul};

use serde::{Deserialize, Serialize};

use crate::TextItem;

use super::data::WEIGHTS_PERMA;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PERMAClass {
    #[serde(rename = "POS_P")]
    PositiveP,
    #[serde(rename = "POS_E")]
    PositiveE,
    #[serde(rename = "POS_R")]
    PositiveR,
    #[serde(rename = "POS_M")]
    PositiveM,
    #[serde(rename = "POS_A")]
    PositiveA,
    #[serde(rename = "NEG_P")]
    NegativeP,
    #[serde(rename = "NEG_E")]
    NegativeE,
    #[serde(rename = "NEG_R")]
    NegativeR,
    #[serde(rename = "NEG_M")]
    NegativeM,
    #[serde(rename = "NEG_A")]
    NegativeA,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PERMAInterpretation {
    Positive,
    Neutral,
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PERMAAnalysis {
    pub positive_emotion: PERMAInterpretation,
    pub engagement: PERMAInterpretation,
    pub relationships: PERMAInterpretation,
    pub meaning: PERMAInterpretation,
    pub accomplishment: PERMAInterpretation,
}

fn establish_interpretation(val: f64) -> PERMAInterpretation {
    match val {
        _ if val > 0.0 => PERMAInterpretation::Positive,
        _ if val < 0.0 => PERMAInterpretation::Negative,
        _ => PERMAInterpretation::Neutral,
    }
}

impl From<HashMap<PERMAClass, f64>> for PERMAAnalysis {
    fn from(value: HashMap<PERMAClass, f64>) -> Self {
        let pos_p = value.get(&PERMAClass::PositiveP).unwrap_or(&0.0);
        let neg_p = value.get(&PERMAClass::NegativeP).unwrap_or(&0.0);

        let pos_e = value.get(&PERMAClass::PositiveE).unwrap_or(&0.0);
        let neg_e = value.get(&PERMAClass::NegativeE).unwrap_or(&0.0);

        let pos_r = value.get(&PERMAClass::PositiveR).unwrap_or(&0.0);
        let neg_r = value.get(&PERMAClass::NegativeR).unwrap_or(&0.0);

        let pos_m = value.get(&PERMAClass::PositiveM).unwrap_or(&0.0);
        let neg_m = value.get(&PERMAClass::NegativeM).unwrap_or(&0.0);

        let pos_a = value.get(&PERMAClass::PositiveA).unwrap_or(&0.0);
        let neg_a = value.get(&PERMAClass::NegativeA).unwrap_or(&0.0);

        PERMAAnalysis {
            positive_emotion: establish_interpretation(pos_p - neg_p),
            engagement: establish_interpretation(pos_e - neg_e),
            relationships: establish_interpretation(pos_r - neg_r),
            meaning: establish_interpretation(pos_m - neg_m),
            accomplishment: establish_interpretation(pos_a - neg_a),
        }
    }
}

pub struct PERMA {
    pub items: HashMap<String, Vec<(PERMAClass, f64)>>,
}

impl PERMA {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        for item in WEIGHTS_PERMA.iter() {
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
    pub fn get_entry(&self, term: &str) -> Option<&Vec<(PERMAClass, f64)>> {
        self.items.get(term)
    }

    #[inline(always)]
    pub fn get_score(&self, item: &TextItem, term: &str) -> Option<Vec<(PERMAClass, f64)>> {
        let word_freqs =
            *item.get_term_frequency(term)? as f64;

        let total_freqs =
            item.word_count as f64;

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
    pub fn run(&self, item: &TextItem) -> PERMAAnalysis {
        item
            .bigram_freqs
            .keys()
            .chain(
                item.word_freqs
                    .keys()
            )
            .filter_map(|word| self.get_score(item, word))
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