use lazy_static::lazy_static;

pub mod data;
pub mod analyzer;

lazy_static! {
    pub static ref EXEC_AGE: analyzer::Age = analyzer::Age::new();
}