use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LexiconWeighted {
    pub term: String,
    pub weight: f64,
}