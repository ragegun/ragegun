pub use text::TextItem;

pub mod analyzers;
pub mod text;
pub mod types;

pub struct Ragegun {
    pub data: TextItem,
}

impl Ragegun {
    pub fn new(text: &str) -> Self {
        Self {
            data: TextItem::new(text),
        }
    }

    #[cfg(feature = "age")]
    pub fn age(&self) -> f64 {
        analyzers::age::EXEC_AGE.run(&self.data)
    }

    #[cfg(feature = "gender")]
    pub fn gender(&self) -> analyzers::gender::analyzer::GenderInterpretation {
        analyzers::gender::EXEC_GENDER.run(&self.data)
    }

    #[cfg(feature = "distress")]
    pub fn distress(&self) -> f64 {
        analyzers::distress::EXEC_DISTRESS.run(&self.data)
    }

    #[cfg(feature = "perma")]
    pub fn perma(&self) -> analyzers::perma::analyzer::PERMAAnalysis {
        analyzers::perma::EXEC_PERMA.run(&self.data)
    }

    #[cfg(feature = "emolex_all_languages")]
    pub fn emolex_all_languages(&self) -> analyzers::emolex_shared::EmoLexEmotions {
        analyzers::emolex_all_languages::EXEC_EMOLEX_ALL_LANGUAGES.run(&self.data)
    }

    #[cfg(feature = "emolex_all_languages")]
    pub fn emolex_english(&self) -> analyzers::emolex_shared::EmoLexEmotions {
        analyzers::emolex_english::EXEC_EMOLEX_ENGLISH.run(&self.data)
    }
}