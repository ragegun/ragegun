use lazy_static::lazy_static;

use crate::types::weighted_with_class::LexiconWeightedWithClass;

use super::analyzer::PERMAClass;

lazy_static! {
    pub static ref WEIGHTS_PERMA: Vec<LexiconWeightedWithClass<PERMAClass>> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/perma.csv")).to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();
}