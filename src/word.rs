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
    pub first_syllable_list : Vec<Weighted<String>>,
    pub last_syllable_list : Vec<Weighted<String>>,
    pub normal_syllable_list : Vec<Weighted<String>>,

    pub graphemes : Vec<(String, Vec<Weighted<String>>)>,

    pub rewrites : RewriteGroup,

    pub rejects : RejectGroup,

    pub settings : GenerateSettings,
}

trait WordGenerator {
//    fn new(
//        &mut self,
//        first_syllables : &Vec<Weighted<String>>,
//        last_syllables : &Vec<Weighted<String>>,
//        normal_syllables : &Vec<Weighted<String>>,
//        graphemes : &Vec<(String, Vec<Weighted<String>>)>,
//        settings: &GenerateSettings);
    fn generate_syllables(&mut self, word: &mut Word);
    fn generate_graphemes(&mut self, word: &mut Word);
    fn rewrite_syllables(&mut self, word: &mut Word);
    fn rewrite_graphemes(&mut self, word: &mut Word);
    fn mark_syllable_rejects(&mut self, word: &mut Word);
    fn mark_grapheme_rejects(&mut self, word: &mut Word);
}

impl WordGenerator for WordFactory {
    fn generate_syllables(&mut self, word: &mut Word) {
        //stuff
    }
    fn generate_graphemes(&mut self, word: &mut Word) {

    }
    fn rewrite_syllables(&mut self, word: &mut Word) {

    }
    fn rewrite_graphemes(&mut self, word: &mut Word) {

    }
    fn mark_syllable_rejects(&mut self, word: &mut Word) {

    }
    fn mark_grapheme_rejects(&mut self, word: &mut Word) {

    }
}
