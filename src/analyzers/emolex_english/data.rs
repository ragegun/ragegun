use lazy_static::lazy_static;

use super::analyzer::EmoLexRaw;

lazy_static! {
    pub static ref WEIGHTS_EMOLEX_ENGLISH: Vec<EmoLexRaw> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/emolex.csv")).to_vec()
        )
        .deserialize::<EmoLexRaw>()
        .filter_map(|i| i.ok())
        .collect();
}