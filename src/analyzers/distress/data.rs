use lazy_static::lazy_static;

use crate::types::rated::LexiconRated;

lazy_static! {
    pub static ref WEIGHTS_DISTRESS: Vec<LexiconRated> =
        csv::Reader::from_reader(
            &*include_bytes!(concat!(env!("ASSET_DIR"), "/distress.csv")).to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();
}