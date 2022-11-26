use lazy_static::lazy_static;

use crate::types::weighted_with_class::LexiconWeightedWithClass;

use super::analyzer::AffectionClass;

lazy_static! {
    pub static ref WEIGHTS_AFFECTION: Vec<LexiconWeightedWithClass<AffectionClass>> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/affection.csv")).to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();
}