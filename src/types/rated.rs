use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LexiconRated {
    pub term: String,
    pub rating: f32,
}