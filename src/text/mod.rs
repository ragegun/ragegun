use std::collections::{HashMap, HashSet};
use std::io::BufReader;

use lazy_static::lazy_static;
use nlprule::tokenizer::Tokenizer;

lazy_static! {
    pub(crate) static ref EN_TOKENIZER: Tokenizer = {
        let model = include_bytes!(concat!(env!("ASSET_DIR"), "/en_tokenizer.bin"));

        Tokenizer::from_reader(BufReader::new(&*model.to_vec())).unwrap()
    };
}

#[derive(Debug, Clone)]
pub struct TextItem {
    pub text: String,
    pub clean_text: String,
    pub sentence_words: Vec<Vec<String>>,
    pub bigrams: Vec<String>,
    pub word_freqs: HashMap<String, usize>,
    pub word_freq_avg: f64,
    pub word_count: u64,
    pub bigram_freqs: HashMap<String, usize>,
    pub bigram_freq_avg: f64,
    pub bigram_count: u64,

    pub filter: Option<HashSet<String>>,
}

impl TextItem {
    pub fn new(text: &str) -> Self {
        let mut item = Self {
            text: text.to_lowercase(),
            clean_text: String::new(),
            sentence_words: vec![],
            bigrams: vec![],
            word_freqs: HashMap::new(),
            word_freq_avg: 0.0,
            word_count: 0,
            bigram_freqs: HashMap::new(),
            bigram_freq_avg: 0.0,
            bigram_count: 0,
            filter: None,
        };

        item.init();

        item
    }

    pub fn new_with_filter(text: &str, filter: HashSet<String>) -> Self {
        let mut item = Self {
            text: text.to_lowercase(),
            clean_text: String::new(),
            sentence_words: vec![],
            bigrams: vec![],
            word_freqs: HashMap::new(),
            word_freq_avg: 0.0,
            word_count: 0,
            bigram_freqs: HashMap::new(),
            bigram_freq_avg: 0.0,
            bigram_count: 0,
            filter: Some(filter),
        };

        item.init();

        item
    }

    pub fn init(&mut self) {
        self.text_remove_special_characters();
        self.extract_sentence_words();
        self.calculate_word_frequencies();
        self.calculate_bigrams();
        self.calculate_bigram_frequencies();
    }

    #[inline(always)]
    pub fn calculate_bigrams(&mut self) {
        let mut bigrams = vec![];

        for sentence in self.sentence_words.iter() {
            for i in 0..sentence.len() - 1 {
                if i + 1 > sentence.len() {
                    break;
                }

                let bigram = &[
                    &*sentence[i],
                    &*sentence[i + 1],
                ];

                bigrams.push(bigram.join(" "));
            }
        }

        self.bigrams = bigrams;
    }

    #[inline(always)]
    pub fn calculate_word_frequencies(&mut self) {
        let mut wordfreqs = HashMap::new();

        let mut word_count = 0;

        for word in self.sentence_words.iter().flatten() {
            let count = wordfreqs.entry(word.to_string()).or_insert(0);

            *count += 1;
            word_count += 1;
        }

        self.word_freq_avg = wordfreqs.values().sum::<usize>() as f64 / wordfreqs.len() as f64;
        self.word_freqs = wordfreqs;
        self.word_count = word_count;
    }

    #[inline(always)]
    pub fn calculate_bigram_frequencies(&mut self) {
        let mut bigramfreqs = HashMap::new();

        let mut bigram_count = 0;

        for bigram in self.bigrams.iter() {
            let count =
                bigramfreqs
                    .entry(
                        bigram.to_string()
                    )
                    .or_insert(0);

            *count += 1;
            bigram_count += 1;
        }

        self.bigram_freq_avg = bigramfreqs.values().sum::<usize>() as f64 / bigramfreqs.len() as f64;
        self.bigram_freqs = bigramfreqs;
        self.bigram_count = bigram_count;
    }

    #[inline(always)]
    pub fn text_remove_special_characters(&mut self) {
        self.clean_text =
            self.text
                .chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect();
    }

    #[inline(always)]
    pub fn extract_sentence_words(&mut self) {
        self.sentence_words =
            EN_TOKENIZER
                .pipe(
                    &self.text
                )
                .map(|x|
                    x
                        .tokens()
                        .iter()
                        .map(|i|
                            i
                                .word()
                                .text()
                                .as_str()
                                .to_string()
                        ).collect::<Vec<_>>()
                )
                .collect();
    }

    #[inline(always)]
    pub fn get_sentence_words(&self) -> &Vec<Vec<String>> {
        &self.sentence_words
    }

    #[inline(always)]
    pub fn get_term_frequency(&self, term: &str) -> Option<&usize> {
        self.word_freqs
            .get(term)
            .or_else(|| self.bigram_freqs.get(term))
    }

    #[inline(always)]
    pub fn words(&self) -> Vec<&String> {
        self.sentence_words
            .iter()
            .flatten()
            .collect()
    }
}