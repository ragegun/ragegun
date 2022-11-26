use std::collections::HashMap;
use std::ops::{Div, Mul};

use serde::{Deserialize, Serialize};

use crate::TextItem;

use super::data::WEIGHTS_EMOLEX_ALL_LANGUAGES;
use super::super::emolex_shared::{EmoLexEmotions, EmoLexEmotionsRaw};
use super::super::emolex_shared::EmoLexEmotion;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmoLexRaw {
    english: String,
    anger: u8,
    anticipation: u8,
    disgust: u8,
    fear: u8,
    joy: u8,
    negative: u8,
    positive: u8,
    sadness: u8,
    surprise: u8,
    trust: u8,
    afrikaans: String,
    albanian: String,
    amharic: String,
    arabic: String,
    armenian: String,
    azerbaijani: String,
    basque: String,
    belarusian: String,
    bengali: String,
    bosnian: String,
    bulgarian: String,
    catalan: String,
    cebuano: String,
    chichewa: String,
    #[serde(rename = "chinese-simplified")]
    chinese_simplified: String,
    #[serde(rename = "chinese-traditional")]
    chinese_traditional: String,
    corsican: String,
    croatian: String,
    czech: String,
    danish: String,
    dutch: String,
    esperanto: String,
    estonian: String,
    filipino: String,
    finnish: String,
    french: String,
    frisian: String,
    galician: String,
    georgian: String,
    german: String,
    greek: String,
    gujarati: String,
    #[serde(rename = "haitian-creole")]
    haitian_creole: String,
    hausa: String,
    hawaiian: String,
    hebrew: String,
    hindi: String,
    hmong: String,
    hungarian: String,
    icelandic: String,
    igbo: String,
    indonesian: String,
    irish: String,
    italian: String,
    japanese: String,
    javanese: String,
    kannada: String,
    kazakh: String,
    khmer: String,
    kinyarwanda: String,
    korean: String,
    #[serde(rename = "kurdish-kurmanji")]
    kurdish_kurmanji: String,
    kyrgyz: String,
    lao: String,
    latin: String,
    latvian: String,
    lithuanian: String,
    luxembourgish: String,
    macedonian: String,
    malagasy: String,
    malay: String,
    malayalam: String,
    maltese: String,
    maori: String,
    marathi: String,
    mongolian: String,
    #[serde(rename = "myanmar-burmese")]
    myanmar_burmese: String,
    nepali: String,
    norwegian: String,
    odia: String,
    pashto: String,
    persian: String,
    polish: String,
    portuguese: String,
    punjabi: String,
    romanian: String,
    russian: String,
    samoan: String,
    #[serde(rename = "scots-gaelic")]
    scots_gaelic: String,
    serbian: String,
    sesotho: String,
    shona: String,
    sindhi: String,
    sinhala: String,
    slovak: String,
    slovenian: String,
    somali: String,
    spanish: String,
    sundanese: String,
    swahili: String,
    swedish: String,
    tajik: String,
    tamil: String,
    tatar: String,
    telugu: String,
    thai: String,
    turkish: String,
    turkmen: String,
    ukrainian: String,
    urdu: String,
    uyghur: String,
    uzbek: String,
    vietnamese: String,
    welsh: String,
    xhosa: String,
    yiddish: String,
    yoruba: String,
    zulu: String,
}

impl EmoLexRaw {
    pub fn get_emotions(&self) -> Vec<EmoLexEmotion> {
        let mut emotions = Vec::new();

        if self.anger == 1 {
            emotions.push(EmoLexEmotion::Anger)
        }

        if self.anticipation == 1 {
            emotions.push(EmoLexEmotion::Anticipation)
        }

        if self.disgust == 1 {
            emotions.push(EmoLexEmotion::Disgust)
        }

        if self.fear == 1 {
            emotions.push(EmoLexEmotion::Fear)
        }

        if self.joy == 1 {
            emotions.push(EmoLexEmotion::Joy)
        }

        if self.negative == 1 {
            emotions.push(EmoLexEmotion::Negative)
        }

        if self.positive == 1 {
            emotions.push(EmoLexEmotion::Positive)
        }

        if self.sadness == 1 {
            emotions.push(EmoLexEmotion::Sadness)
        }

        if self.surprise == 1 {
            emotions.push(EmoLexEmotion::Surprise)
        }

        if self.trust == 1 {
            emotions.push(EmoLexEmotion::Trust)
        }

        emotions
    }

    pub fn transform(self) -> (Vec<String>, Vec<EmoLexEmotion>) {
        let emotions = self.get_emotions();

        let words = vec![
            self.english,
            self.afrikaans,
            self.albanian,
            self.amharic,
            self.arabic,
            self.armenian,
            self.azerbaijani,
            self.basque,
            self.belarusian,
            self.bengali,
            self.bosnian,
            self.bulgarian,
            self.catalan,
            self.cebuano,
            self.chichewa,
            self.chinese_simplified,
            self.chinese_traditional,
            self.corsican,
            self.croatian,
            self.czech,
            self.danish,
            self.dutch,
            self.esperanto,
            self.estonian,
            self.filipino,
            self.finnish,
            self.french,
            self.frisian,
            self.galician,
            self.georgian,
            self.german,
            self.greek,
            self.gujarati,
            self.haitian_creole,
            self.hausa,
            self.hawaiian,
            self.hebrew,
            self.hindi,
            self.hmong,
            self.hungarian,
            self.icelandic,
            self.igbo,
            self.indonesian,
            self.irish,
            self.italian,
            self.japanese,
            self.javanese,
            self.kannada,
            self.kazakh,
            self.khmer,
            self.kinyarwanda,
            self.korean,
            self.kurdish_kurmanji,
            self.kyrgyz,
            self.lao,
            self.latin,
            self.latvian,
            self.lithuanian,
            self.luxembourgish,
            self.macedonian,
            self.malagasy,
            self.malay,
            self.malayalam,
            self.maltese,
            self.maori,
            self.marathi,
            self.mongolian,
            self.myanmar_burmese,
            self.nepali,
            self.norwegian,
            self.odia,
            self.pashto,
            self.persian,
            self.polish,
            self.portuguese,
            self.punjabi,
            self.romanian,
            self.russian,
            self.samoan,
            self.scots_gaelic,
            self.serbian,
            self.sesotho,
            self.shona,
            self.sindhi,
            self.sinhala,
            self.slovak,
            self.slovenian,
            self.somali,
            self.spanish,
            self.sundanese,
            self.swahili,
            self.swedish,
            self.tajik,
            self.tamil,
            self.tatar,
            self.telugu,
            self.thai,
            self.turkish,
            self.turkmen,
            self.ukrainian,
            self.urdu,
            self.uyghur,
            self.uzbek,
            self.vietnamese,
            self.welsh,
            self.xhosa,
            self.yiddish,
            self.yoruba,
            self.zulu,
        ];

        (
            words,
            emotions,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EMOLEX {
    pub emotions: Vec<&'static Vec<EmoLexEmotion>>,
    pub items: HashMap<&'static str, usize>,
}

impl EMOLEX {
    pub fn new() -> Self {
        let mut emotion_ref = Vec::new();
        let mut items = HashMap::new();

        for (words, emotions) in WEIGHTS_EMOLEX_ALL_LANGUAGES.iter() {
            emotion_ref.push(emotions);

            let emotion = emotion_ref.len() - 1;

            words
                .iter()
                .for_each(|word| {
                    items.insert(word.as_str(), emotion);
                });
        }

        Self {
            emotions: emotion_ref,
            items,
        }
    }

    pub fn get_entry(&self, term: &str) -> Option<&'static Vec<EmoLexEmotion>> {
        Some(
            self.emotions.get(
                *self.items.get(term)?
            )?
        )
    }

    pub fn get_score(&self, item: &TextItem, word: &str) -> Option<Vec<(&'static EmoLexEmotion, f64)>> {
        let word_freqs =
            *item.word_freqs.get(word)? as f64;

        let total_freqs =
            item.word_count as f64;

        Some(
            self.get_entry(word)?
                .iter()
                .map(|emotion|
                    (
                        emotion,
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
                match *emotion {
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