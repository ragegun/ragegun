use std::collections::HashMap;

use lazy_static::lazy_static;

pub mod perma;
pub mod types;
pub mod distress;
pub mod age;
pub mod gender;
pub mod emolex;

type RagegunWeights = HashMap<String, HashMap<String, f32>>;

lazy_static! {
    pub static ref WEIGHTS_AGE: Vec<types::weighted::LexiconWeighted> =
        csv::Reader::from_reader(
            &*include_bytes!("./files/age.csv").to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();

    pub static ref EXEC_AGE: age::Age = age::Age::new();
}

lazy_static! {
    pub static ref WEIGHTS_DISTRESS: Vec<types::rated::LexiconRated> =
        csv::Reader::from_reader(
            &*include_bytes!("./files/distress.csv").to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();

    pub static ref EXEC_DISTRESS: distress::Distress = distress::Distress::new();
}

lazy_static! {
    pub static ref WEIGHTS_EMOLEX: Vec<(Vec<String>, Vec<emolex::EmoLexEmotion>)> =
        csv::Reader::from_reader(
            &*include_bytes!("./files/emolex-full.csv").to_vec()
        )
        .deserialize::<emolex::EmoLexRaw>()
        .filter_map(|i| i.ok())
        .map(|i| i.transform())
        .collect();

    pub static ref EXEC_EMOLEX: emolex::EMOLEX = emolex::EMOLEX::new();
}

lazy_static! {
    pub static ref WEIGHTS_GENDER: Vec<types::weighted::LexiconWeighted> =
        csv::Reader::from_reader(
            &*include_bytes!("./files/gender.csv").to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();

    pub static ref EXEC_GENDER: gender::Gender = gender::Gender::new();
}

lazy_static! {
    pub static ref WEIGHTS_PERMA: Vec<types::weighted_with_class::LexiconWeightedWithClass<perma::PERMAClass>> =
        csv::Reader::from_reader(
            &*include_bytes!("./files/perma.csv").to_vec()
        )
        .deserialize()
        .filter_map(|i| i.ok())
        .collect();

    pub static ref EXEC_PERMA: perma::PERMA = perma::PERMA::new();
}