use std::collections::{HashMap, HashSet};
use std::io::BufReader;

use lazy_static::lazy_static;
use nlprule::tokenizer::Tokenizer;

use analyzers::affection::EXEC_AFFECTION;
use analyzers::age::EXEC_AGE;
use analyzers::distress::EXEC_DISTRESS;
use analyzers::emolex_english::EXEC_EMOLEX_ENGLISH;
use analyzers::gender::EXEC_GENDER;
use analyzers::perma::EXEC_PERMA;

pub mod analyzers;
pub mod types;

lazy_static! {
    pub static ref EN_TOKENIZER: Tokenizer = {
        let model = include_bytes!(concat!(env!("ASSET_DIR"), "/en_tokenizer.bin"));

        Tokenizer::from_reader(BufReader::new(&*model.to_vec())).unwrap()
    };
}

#[derive(Debug, Clone)]
pub struct TextItem {
    pub text: String,
    pub clean_text: String,
    pub sentence_words: Vec<Vec<String>>,
    pub bigrams: Vec<String>,
    pub word_freqs: HashMap<String, usize>,
    pub word_freq_avg: f32,
    pub word_count: u32,
    pub bigram_freqs: HashMap<String, usize>,
    pub bigram_freq_avg: f32,
    pub bigram_count: u32,

    pub filter: Option<HashSet<String>>,
}

impl TextItem {
    pub fn new(text: &str) -> Self {
        let mut item = Self {
            text: text.to_lowercase(),
            clean_text: String::new(),
            sentence_words: vec![],
            bigrams: vec![],
            word_freqs: HashMap::new(),
            word_freq_avg: 0.0,
            word_count: 0,
            bigram_freqs: HashMap::new(),
            bigram_freq_avg: 0.0,
            bigram_count: 0,
            filter: None,
        };

        item.init();

        item
    }

    pub fn new_with_filter(text: &str, filter: HashSet<String>) -> Self {
        let mut item = Self {
            text: text.to_lowercase(),
            clean_text: String::new(),
            sentence_words: vec![],
            bigrams: vec![],
            word_freqs: HashMap::new(),
            word_freq_avg: 0.0,
            word_count: 0,
            bigram_freqs: HashMap::new(),
            bigram_freq_avg: 0.0,
            bigram_count: 0,
            filter: Some(filter),
        };

        item.init();

        item
    }

    pub fn init(&mut self) {
        self.text_remove_special_characters();
        self.extract_sentence_words();
        self.calculate_word_frequencies();
        self.calculate_bigrams();
        self.calculate_bigram_frequencies();
    }

    #[inline(always)]
    pub fn calculate_bigrams(&mut self) {
        let mut bigrams = vec![];

        for sentence in self.sentence_words.iter() {
            for i in 0..sentence.len() - 1 {
                if i + 1 > sentence.len() {
                    break;
                }

                let bigram = &[
                    &*sentence[i],
                    &*sentence[i + 1],
                ];

                bigrams.push(bigram.join(" "));
            }
        }

        self.bigrams = bigrams;
    }

    #[inline(always)]
    pub fn calculate_word_frequencies(&mut self) {
        let mut wordfreqs = HashMap::new();

        let mut word_count = 0;

        for word in self.sentence_words.iter().flatten() {
            let count = wordfreqs.entry(word.to_string()).or_insert(0);

            *count += 1;
            word_count += 1;
        }

        self.word_freq_avg = wordfreqs.values().sum::<usize>() as f32 / wordfreqs.len() as f32;
        self.word_freqs = wordfreqs;
        self.word_count = word_count;
    }

    #[inline(always)]
    pub fn calculate_bigram_frequencies(&mut self) {
        let mut bigramfreqs = HashMap::new();

        let mut bigram_count = 0;

        for bigram in self.bigrams.iter() {
            let count =
                bigramfreqs
                    .entry(
                        bigram.to_string()
                    )
                    .or_insert(0);

            *count += 1;
            bigram_count += 1;
        }

        self.bigram_freq_avg = bigramfreqs.values().sum::<usize>() as f32 / bigramfreqs.len() as f32;
        self.bigram_freqs = bigramfreqs;
        self.bigram_count = bigram_count;
    }

    #[inline(always)]
    pub fn text_remove_special_characters(&mut self) {
        self.clean_text =
            self.text
                .chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect();
    }

    #[inline(always)]
    pub fn extract_sentence_words(&mut self) {
        self.sentence_words =
            EN_TOKENIZER
                .pipe(
                    &self.text
                )
                .map(|x|
                    x
                        .tokens()
                        .iter()
                        .map(|i|
                            i
                                .word()
                                .text()
                                .as_str()
                                .to_string()
                        ).collect::<Vec<_>>()
                )
                .collect();
    }

    #[inline(always)]
    pub fn get_sentence_words(&self) -> &Vec<Vec<String>> {
        &self.sentence_words
    }

    #[inline(always)]
    pub fn words(&self) -> Vec<&String> {
        self.sentence_words
            .iter()
            .flatten()
            .collect()
    }
}

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
    //let text = r#"He ghosted me over my birthday and left the country, I found out on instagram using a fake account because he had blocked me. he sent some sporadic and impersonal emails saying "miss u" and "see u when I'm back", when he came back he did not reach out. a couple days passed, he reached out asking to "hang out" (mind you, this is my boyfriend!), I suggested when, he ghosted again, a few days later said he went on a drug bender. not to make this too long, weeks later, we finally met today. in a bar. He asked me how I am, to which I responded "very good and also very bad" (implying because of our situation). he didnt like that and said I have malicious intent. He said he doesnt like my face, he said I would smirk and make fun of him. I denied all of this obviously. He went on to insult me more, told me I am 30 years old and never worked a day in my life (he doesnt consider working at a university a real job), the time we spent with each other was all bad, I disrespect him to the fullest, I fucked this one up, he didnt think our meeting would end up like this. He also told me we are too similar and he doesnt want to date himself (i highly doubt we are similar like that). I dont know how we can be so similar if i am a loser and hes successful but okay. I also tried to be nice and told him I cherished our time together and that we could stay friends and that way he could find out that i am nothing like he thinks i am. that it would be a safe way to get to know me better. he said no and that bitches always keep their exes in their back pocket, we wont be on the back burner. he said I just want to keep him in my portfolio like a leech who is collecting successful people. I also told him that i loved him so much and that i would have married him to which he replied "shit that girls say" and that i shouldnt be so corny and generic. he also called me mentally instable. I asked for my stuff back. He said not today. I said its not a question, I need it today. He said the more i ask the less he will give it to me. I said I can call the cops and get my stuff. He said he will mail it. I said when and that i dont trust him to send it. He said stop herassing me about this and punched me in the head, hard. I heard a peeping noise. I ran across the street. A guy called the cops. they took him back to his place and extracted my valuables. They made me go to the ER to document the incident. My eardrum now has a rip in it. I don't even know what to say. I am very very sad. he hates me so much. there was 0,00% love in any interaction we just shared. just pure hate towards me. i wanted to be with this guy so hard and was super committed. i didnt do anything to betray him. and it seemed like he took great pleasure in making me his literal punching bag."#;
    let text = r#"That's it then. They destroyed and changed me, I don't even recognize myself anymore. I became violent and hurt those around me. I recognized the best thing I can do is to move away and start a new life. It's hard to get off them and it's the worst to have them lying around. I keep telling myself not to use them but they're calling for me. I don't think I can go back being how I was before them because I've done terrible things and I still feel like simply cannot appreciate the people around me at all nor enjoy simple things anymore. I have this massive drive to get high no matter what I do, which in itself causes derealisation in me. Is this question urgent for you because you're in danger? Is this your boyfriend?"#;

    let _rg = Ragegun::new();

    let ti = TextItem::new(text);

    dbg!(EXEC_AGE.run(&ti));
    dbg!(EXEC_GENDER.run(&ti));
    dbg!(EXEC_DISTRESS.run(&ti));
    dbg!(EXEC_PERMA.run(&ti));
    dbg!(EXEC_EMOLEX_ENGLISH.run(&ti));
    dbg!(EXEC_AFFECTION.run(&ti));
}
