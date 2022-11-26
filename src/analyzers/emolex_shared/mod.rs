use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum EmolexInterpretation {
    Low,
    Moderate,
    Neutral,
    High,
    VeryHigh,
    #[default]
    Unknown,
}

impl From<f64> for EmolexInterpretation {
    fn from(value: f64) -> Self {
        match value {
            _ if value > 0.0 && value < 0.1 => EmolexInterpretation::Low,
            _ if value < 0.3 => EmolexInterpretation::Moderate,
            _ if value < 0.5 => EmolexInterpretation::Neutral,
            _ if value < 0.7 => EmolexInterpretation::High,
            _ if value < 0.9 => EmolexInterpretation::VeryHigh,
            _ => EmolexInterpretation::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum EmoLexEmotion {
    Anger,
    Anticipation,
    Disgust,
    Fear,
    Joy,
    Negative,
    Positive,
    Sadness,
    Surprise,
    Trust,
    Unknown,
}

impl From<String> for EmoLexEmotion {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "anger" => EmoLexEmotion::Anger,
            "anticipation" => EmoLexEmotion::Anticipation,
            "disgust" => EmoLexEmotion::Disgust,
            "fear" => EmoLexEmotion::Fear,
            "joy" => EmoLexEmotion::Joy,
            "negative" => EmoLexEmotion::Negative,
            "positive" => EmoLexEmotion::Positive,
            "sadness" => EmoLexEmotion::Sadness,
            "surprise" => EmoLexEmotion::Surprise,
            "trust" => EmoLexEmotion::Trust,
            _ => EmoLexEmotion::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmoLexEmotions {
    pub anger: EmolexInterpretation,
    pub anticipation: EmolexInterpretation,
    pub disgust: EmolexInterpretation,
    pub fear: EmolexInterpretation,
    pub joy: EmolexInterpretation,
    pub negative: EmolexInterpretation,
    pub positive: EmolexInterpretation,
    pub sadness: EmolexInterpretation,
    pub surprise: EmolexInterpretation,
    pub trust: EmolexInterpretation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmoLexEmotionsRaw {
    pub anger: f64,
    pub anticipation: f64,
    pub disgust: f64,
    pub fear: f64,
    pub joy: f64,
    pub negative: f64,
    pub positive: f64,
    pub sadness: f64,
    pub surprise: f64,
    pub trust: f64,
}

impl From<EmoLexEmotionsRaw> for EmoLexEmotions {
    fn from(raw: EmoLexEmotionsRaw) -> Self {
        EmoLexEmotions {
            anger: raw.anger.into(),
            anticipation: raw.anticipation.into(),
            disgust: raw.disgust.into(),
            fear: raw.fear.into(),
            joy: raw.joy.into(),
            negative: raw.negative.into(),
            positive: raw.positive.into(),
            sadness: raw.sadness.into(),
            surprise: raw.surprise.into(),
            trust: raw.trust.into(),
        }
    }
}