#[cfg(feature = "age")]
pub mod age;

#[cfg(feature = "affection")]
pub mod affection;

#[cfg(feature = "perma")]
pub mod perma;

#[cfg(feature = "distress")]
pub mod distress;

#[cfg(feature = "gender")]
pub mod gender;

#[cfg(feature = "emolex_all_languages")]
pub mod emolex_all_languages;

#[cfg(feature = "emolex_english")]
pub mod emolex_english;

#[cfg(feature = "emolex_all_languages")]
#[cfg(feature = "emolex_english")]
pub mod emolex_shared;