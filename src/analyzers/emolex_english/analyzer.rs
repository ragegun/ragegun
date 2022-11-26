use std::collections::HashMap;
use std::ops::{Div, Mul};

use serde::{Deserialize, Serialize};

use crate::TextItem;

use super::data::WEIGHTS_EMOLEX_ENGLISH;
use super::super::emolex_shared::{EmoLexEmotion, EmoLexEmotions, EmoLexEmotionsRaw};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmoLexRaw {
    term: String,
    class: String,
    weight: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EMOLEX {
    pub items: HashMap<String, Vec<EmoLexEmotion>>,
}

impl EMOLEX {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        for entry in WEIGHTS_EMOLEX_ENGLISH.iter() {
            items
                .entry(entry.term.clone())
                .or_insert_with(Vec::new)
                .push(entry.class.clone().into());
        }

        Self {
            items,
        }
    }

    pub fn get_entry(&self, term: &str) -> Option<&Vec<EmoLexEmotion>> {
        Some(
            self
                .items
                .get(term)?,
        )
    }

    pub fn get_score(&self, item: &TextItem, word: &str) -> Option<Vec<(EmoLexEmotion, f32)>> {
        let word_freqs =
            *item.word_freqs.get(word)? as f32;

        let total_freqs =
            item.word_count as f32;

        Some(
            self.get_entry(word)?
                .iter()
                .map(|emotion|
                    (
                        *emotion,
                        word_freqs.div(total_freqs).mul(word_freqs)
                    )
                )
                .collect(),
        )
    }

    pub fn run(&self, item: &TextItem) -> EmoLexEmotions {
        item.word_freqs
            .keys()
            .filter_map(|word| self.get_score(item, word))
            .flatten()
            .fold(EmoLexEmotionsRaw::default(), |mut acc, (emotion, score)| {
                match emotion {
                    EmoLexEmotion::Anger => { acc.anger += score; }
                    EmoLexEmotion::Anticipation => { acc.anticipation += score; }
                    EmoLexEmotion::Disgust => { acc.disgust += score; }
                    EmoLexEmotion::Fear => { acc.fear += score; }
                    EmoLexEmotion::Joy => { acc.joy += score; }
                    EmoLexEmotion::Negative => { acc.negative += score; }
                    EmoLexEmotion::Positive => { acc.positive += score; }
                    EmoLexEmotion::Sadness => { acc.sadness += score; }
                    EmoLexEmotion::Surprise => { acc.surprise += score; }
                    EmoLexEmotion::Trust => { acc.trust += score; }
                    EmoLexEmotion::Unknown => unimplemented!()
                }

                acc
            })
            .into()
    }
}