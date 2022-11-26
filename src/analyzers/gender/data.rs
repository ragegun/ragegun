use lazy_static::lazy_static;

use crate::types::weighted::LexiconWeighted;

lazy_static! {
    pub static ref WEIGHTS_GENDER: Vec<LexiconWeighted> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/gender.csv")).to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();
}