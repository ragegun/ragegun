use std::collections::HashMap;
use std::ops::{Div, Mul};

use serde::{Deserialize, Serialize};

use crate::TextItem;

use super::data::WEIGHTS_AFFECTION;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum AffectionClass {
    #[serde(rename = "AFFECT_AVG")]
    Affection,
    #[serde(rename = "INTENSITY_AVG")]
    Intensity,
}

pub const INTERCEPTS_AFFECTION: f64 = 5.03710472069;
pub const INTERCEPTS_INTENSITY: f64 = 2.39976263142;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum AffectionInterpretation {
    Low,
    Moderate,
    Neutral,
    High,
    VeryHigh,
    Unknown,
}

impl From<f64> for AffectionInterpretation {
    fn from(value: f64) -> Self {
        match value {
            _ if value > 0.0 && value < 1.0 => AffectionInterpretation::Low,
            _ if value < 3.0 => AffectionInterpretation::Moderate,
            _ if value < 5.0 => AffectionInterpretation::Neutral,
            _ if value < 7.0 => AffectionInterpretation::High,
            _ if value < 9.0 => AffectionInterpretation::VeryHigh,
            _ => AffectionInterpretation::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectionAnalysis {
    pub affection: AffectionInterpretation,
    pub intensity: AffectionInterpretation,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectionAnalysisRaw {
    pub affection: f64,
    pub intensity: f64,
}

impl AffectionAnalysisRaw {
    pub fn new() -> Self {
        Self {
            affection: 0.0,
            intensity: 0.0,
        }
    }

    pub fn apply_intercepts(&mut self) {
        self.affection += INTERCEPTS_AFFECTION;
        self.intensity += INTERCEPTS_INTENSITY;
    }
}

impl From<AffectionAnalysisRaw> for AffectionAnalysis {
    fn from(value: AffectionAnalysisRaw) -> Self {
        Self {
            affection: value.affection.into(),
            intensity: value.intensity.into(),
        }
    }
}

pub struct Affection {
    pub items: HashMap<String, Vec<(AffectionClass, f64)>>,
}

impl Affection {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        for item in WEIGHTS_AFFECTION.iter() {
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
    pub fn get_entry(&self, term: &str) -> Option<&Vec<(AffectionClass, f64)>> {
        self.items.get(term)
    }

    #[inline(always)]
    pub fn get_score(&self, item: &TextItem, word: &str) -> Option<Vec<(AffectionClass, f64)>> {
        let word_freq = (*item.word_freqs.get(word)?) as f64;
        let total_freqs = item.word_freqs.len() as f64;

        Some(
            self.get_entry(word)?
                .iter()
                .map(|(class, weight)| {
                    (
                        *class,
                        word_freq
                            .div(total_freqs)
                            .mul(weight)
                    )
                })
                .collect(),
        )
    }

    #[inline(always)]
    pub fn run(&self, item: &TextItem) -> AffectionAnalysis {
        let mut analysis =
            item.word_freqs
                .keys()
                .filter_map(|word| self.get_score(item, word))
                .flatten()
                .fold(AffectionAnalysisRaw::new(), |mut acc, (class, score)| {
                    match class {
                        AffectionClass::Affection => acc.affection += score,
                        AffectionClass::Intensity => acc.intensity += score,
                    }

                    acc
                });

        analysis.apply_intercepts();

        analysis.into()
    }
}