use lazy_static::lazy_static;

use crate::types::weighted_with_class::LexiconWeightedWithClass;

use super::analyzer::TemporalOrientationClass;

lazy_static! {
    pub static ref WEIGHTS_TEMPORAL: Vec<LexiconWeightedWithClass<TemporalOrientationClass>> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/temporal.csv")).to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();
}