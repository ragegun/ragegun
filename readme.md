# ragegun

Lexica based text analysis.

As crates.io is a pain in the ass, this module will download the lexica from GitHub at build time.

## API

### Age (default feature, age)

```rust
mod ragegun;

fn main() {
    let rg = ragegun::RageGun::new("foo");
    
    dbg!(rg.age()); // value like 30.00
}
```

### Gender (default feature, gender)

```rust
mod ragegun;

fn main() {
    let rg = ragegun::RageGun::new("foo");
    
    rg.gender() // value like GenderInterpretation::Female
}
```

### Distress (default feature, distress)
    
```rust
mod ragegun;

fn main() {
    let rg = ragegun::RageGun::new("foo");
    
    dbg!(rg.distress()); // value like 1.00
}
```

### PERMA (default feature, perma)

```rust
mod ragegun;

fn main() {
    let rg = ragegun::RageGun::new("foo");
    
    dbg!(rg.perma());
    
    // PERMAAnalysis {
    //  positive_emotion: Negative,
    //  engagement: Negative,
    //  relationships: Postive,
    //  meaning: Negative,
    //  accomplishment: Negative,
    // }
}
```

### EmoLex (emolex_all_languages, 20MB+)

```rust
mod ragegun;

fn main() {
    let rg = ragegun::RageGun::new("foo");

    dbg!(rg.emolex_all_languages());

    // EmoLexEmotions {
    // anger: Neutral,
    // anticipation: Neutral,
    // disgust: Neutral,
    // fear: Neutral,
    // joy: Neutral,
    // negative: Neutral,
    // positive: Neutral,
    // sadness: Neutral,
    // surprise: Neutral,
    // trust: Neutral,
    // }
}
```

### EmoLex (emolex_english, ~2MB)

```rust
mod ragegun;

fn main() {
    let rg = ragegun::RageGun::new("foo");

    dbg!(rg.emolex_english());

    // EmoLexEmotions {
    // anger: Neutral,
    // anticipation: Neutral,
    // disgust: Neutral,
    // fear: Neutral,
    // joy: Neutral,
    // negative: Neutral,
    // positive: Neutral,
    // sadness: Neutral,
    // surprise: Neutral,
    // trust: Neutral,
    // }
}
```

## References

### Age & gender prediction

- [Schwartz, H. A., Eichstaedt, J. C., Kern, M. L., Dziurzynski, L., Ramones, S. M., Agrawal, M., Shah, A., Kosinski, M., Stillwell, D., Seligman, M. E., & Ungar, L. H. (2013). Personality, Gender, and Age in the Language of Social Media: The Open-Vocabulary Approach. PLOS ONE, 8(9), e73791.](http://journals.plos.org/plosone/article/file?id=10.1371/journal.pone.0073791&type=printable)

### Personal distress prediction

- [João Sedoc, Sven Buechel, Yehonathan Nachmany, Anneke Buffone, & Lyle Ungar (2019). Learning Word Ratings for Empathy and Distress from Document-Level User ](https://arxiv.org/abs/1912.01079)

### Prospection Lexicon: Temporal Orientation

- [Schwartz, H. A., Park, G., Sap, M., Weingarten, E., Eichstaedt, J., Kern, M., Stillwell, D., Kosinski, M., Berger, J., Seligman, M., & Ungar, L. (2015). Extracting Human Temporal Orientation from Facebook Language. NAACL-2015: Conference of the North American Chapter of the Association for Computational Linguistics.](http://www.seas.upenn.edu/~hansens/tempor-naacl15-cr.pdf).

- [Park, G., Schwartz, H.A., Sap, M., Kern, M.L., Weingarten, E., Eichstaedt, J.C., Berger, J., Stillwell, D.J., Kosinski, M., Ungar, L.H. & Seligman, M.E. (2015). Living in the Past, Present, and Future: Measuring Temporal Orientation with Language. Journal of personality.](http://wwbp.org/papers/Park_et_al-2016-Journal_of_Personality.pdf).

### Dark Triad

- [Preotiuc-Pietro, Daniel and Carpenter, Jordan and Giorgi, Salvatore and Ungar, Lyle, {CIKM} (2016). Studying the Dark Triad of Personality using Twitter Behavior. Proceedings of the 25th {ACM} Conference on Information and Knowledge.](http://wwbp.org/papers/darktriad16cikm.pdf)

### Empathic Concern

- [João Sedoc, Sven Buechel, Yehonathan Nachmany, Anneke Buffone, & Lyle Ungar (2019). Learning Word Ratings for Empathy and Distress from Document-Level User ](https://arxiv.org/abs/1912.01079)

### EmoLex / NRC Word-Emotion Association Lexicon

- [Mohammad, S., & Turney, P. (2013). Crowdsourcing a Word-Emotion Association Lexicon. Computational Intelligence, 29(3), 436–465.](http://arxiv.org/pdf/1308.6297.pdf)

- [Mohammad, S., & Turney, P. (2010). Emotions evoked by common words and phrases: Using mechanical turk to create an emotion lexicon. In Proceedings of the NAACL HLT 2010 workshop on computational approaches to analysis and generation of emotion in text (pp. 26–34).](http://saifmohammad.com/WebDocs/Mohammad-Turney-NAACL10-EmotionWorkshop.pdf)

### Affect Intensity / NRC Emotion Intensity Lexicon

- [Mohammad, S. (2018). Word Affect Intensities. In Proceedings of the 11th Edition of the Language Resources and Evaluation Conference (LREC-2018).](http://saifmohammad.com/WebDocs/lrec2018-paper-word-emotion.pdf)

### NRC Valence, Arousal, Dominance Lexicon

- [Mohammad, S. (2018). Obtaining Reliable Human Ratings of Valence, Arousal, and Dominance for 20,000 English Words. In Proceedings of The Annual Conference of the Association for Computational Linguistics (ACL).](http://saifmohammad.com/WebDocs/acl2018-VAD.pdf)

### PERMA (Positive Emotions, Engagement, Relationships, Meaning, Accomplishment)

- [Schwartz, & Ungar, L. (2016). Predicting Individual Well-Being Through the Language of Social Media. Pacific Symposium on Biocomputing, 21, 516-527.](http://wwbp.org/papers/2016_predicting_wellbeing.pdf)

## Sources

- [WWBP](https://www.wwbp.org/lexica.html) (Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported)
- [NRC Word-Emotion Association Lexicon (EmoLex)](https://saifmohammad.com/WebPages/NRC-Emotion-Lexicon.htm)
- [NRC Emotion Intensity Lexicon (NRC-EIL)](http://saifmohammad.com/WebPages/AffectIntensity.htm)
- [NRC Valence, Arousal, and Dominance (NRC-VAD)](http://saifmohammad.com/WebPages/nrc-vad.html)