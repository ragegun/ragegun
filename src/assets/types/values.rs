use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnalysisInterpretation {
    Neutral,
    Positive,
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenderInterpretation {
    Male,
    Female,
    Unknown,
}