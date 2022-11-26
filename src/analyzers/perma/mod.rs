use lazy_static::lazy_static;

pub mod analyzer;
pub mod data;

lazy_static! {
    pub static ref EXEC_PERMA: analyzer::PERMA = analyzer::PERMA::new();
}