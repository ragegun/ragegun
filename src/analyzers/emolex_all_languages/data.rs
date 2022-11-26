use lazy_static::lazy_static;

use super::analyzer::EmoLexRaw;
use super::super::emolex_shared::EmoLexEmotion;

lazy_static! {
    pub static ref WEIGHTS_EMOLEX_ALL_LANGUAGES: Vec<(Vec<String>, Vec<EmoLexEmotion>)> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/emolex-full.csv")).to_vec()
        )
        .deserialize::<EmoLexRaw>()
        .filter_map(|i| i.ok())
        .map(|i| i.transform())
        .collect();
}