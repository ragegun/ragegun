use analyzers::affection::EXEC_AFFECTION;
use analyzers::age::EXEC_AGE;
use analyzers::distress::EXEC_DISTRESS;
use analyzers::emolex_english::EXEC_EMOLEX_ENGLISH;
use analyzers::gender::EXEC_GENDER;
use analyzers::perma::EXEC_PERMA;
use analyzers::temporal_orientation::EXEC_TEMPORAL;
use text::TextItem;

use crate::analyzers::temporal_orientation::analyzer::TemporalOrientationClass;
use crate::types::weighted_with_class::LexiconWeightedWithClass;

pub mod analyzers;
pub mod types;
pub mod text;

pub struct Ragegun {
    //data: RagegunData,
}

impl Ragegun {
    pub fn new() -> Self {
        Self {
            //data: RAGEGUN_DATA.clone(),
        }
    }
}

fn main() {
    let text = r#"In the future, everyone will be able to travel to space. space travel will be safe and common, and there will be many different ways to travel to space. There will be hotels in space, and people will be able to live and work in space. space will be a popular destination for vacations and business trips."#;

    let _rg = Ragegun::new();

    let ti = TextItem::new(text);

    dbg!(EXEC_AGE.run(&ti));
    dbg!(EXEC_GENDER.run(&ti));
    dbg!(EXEC_DISTRESS.run(&ti));
    dbg!(EXEC_PERMA.run(&ti));
    dbg!(EXEC_EMOLEX_ENGLISH.run(&ti));
    dbg!(EXEC_AFFECTION.run(&ti));
    dbg!(EXEC_TEMPORAL.run(&ti));
}
