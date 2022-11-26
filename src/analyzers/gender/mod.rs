use lazy_static::lazy_static;

pub mod analyzer;
pub mod data;

lazy_static! {
    pub static ref EXEC_GENDER: analyzer::Gender = analyzer::Gender::new();
}