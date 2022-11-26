use lazy_static::lazy_static;

use crate::types::weighted::LexiconWeighted;

lazy_static! {
    pub static ref WEIGHTS_AGE: Vec<LexiconWeighted> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/age.csv")).to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();
}