use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LexiconWeightedWithClass<T> {
    pub term: String,
    pub class: T,
    pub weight: f64,
}