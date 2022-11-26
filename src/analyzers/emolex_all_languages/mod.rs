use lazy_static::lazy_static;

pub mod analyzer;
pub mod data;

lazy_static! {
    pub static ref EXEC_EMOLEX_ALL_LANGUAGES: analyzer::EMOLEX = analyzer::EMOLEX::new();
}