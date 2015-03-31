use config::*;
use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

pub struct Word {
    pub syllables : String,
    pub graphemes : String,
    pub syllable_rewrite_history : Vec<(String, String)>,
    pub grapheme_rewrite_history : Vec<(String, String)>,
    pub syllable_rejects : Vec<String>,
    pub grapheme_rejects : Vec<String>,
}

pub struct WordFactory {
    first_syllable_list : Vec<Weighted<String>>,
    last_syllable_list : Vec<Weighted<String>>,
    normal_syllable_list : Vec<Weighted<String>>,

    graphemes : Vec<(String, Vec<Weighted<String>>)>,

    syllable_rewrites : Vec<Rewrite>,
    grapheme_rewrites : Vec<Rewrite>,

    syllable_rejects : Vec<String>,
    grapheme_rejects : Vec<String>,

    settings : GenerateSettings,
}

trait WordGenerator {
    fn new(
        &mut self,
        first_syllables : &Vec<Weighted<String>>,
        last_syllables : &Vec<Weighted<String>>,
        normal_syllables : &Vec<Weighted<String>>,
        graphemes : &Vec<(String, Vec<Weighted<String>>)>,
        settings: &GenerateSettings);
    fn generate_syllables(&mut self, word: &mut Word);
    fn generate_graphemes(&mut self, word: &mut Word);
    fn rewrite_syllables(&mut self, word: &mut Word);
    fn rewrite_graphemes(&mut self, word: &mut Word);
    fn mark_syllable_rejects(&mut self, word: &mut Word);
    fn mark_grapheme_rejects(&mut self, word: &mut Word);
}
