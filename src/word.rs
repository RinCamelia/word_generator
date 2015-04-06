extern crate rand;

use config::*;
use rand::*;
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

pub trait WordGenerator {
//    fn new(
//        &mut self,
//        first_syllables : &Vec<Weighted<String>>,
//        last_syllables : &Vec<Weighted<String>>,
//        normal_syllables : &Vec<Weighted<String>>,
//        graphemes : &Vec<(String, Vec<Weighted<String>>)>,
//        settings: &GenerateSettings);
    fn generate_syllables(&self, word: &mut Word);
    fn generate_graphemes(&self, word: &mut Word);
    fn rewrite_syllables(&self, word: &mut Word);
    fn rewrite_graphemes(&self, word: &mut Word);
    fn mark_syllable_rejects(&self, word: &mut Word);
    fn mark_grapheme_rejects(&self, word: &mut Word);
}

impl WordGenerator for WordFactory {
    fn generate_syllables(&self, word: &mut Word) {
        let mut rng = rand::thread_rng();

        let mut current_chance_for_syllable : f32 = 1.0;
        let mut syllable_count : usize = 0;
        let mut syllable_loop_iter = rng.gen_iter::<f32>();

        loop {
            let result = syllable_loop_iter.next();
            match result {
                Some(x) => {
                    if x < current_chance_for_syllable && syllable_count < self.settings.max_syllables {
                        syllable_count = syllable_count + 1;
                        current_chance_for_syllable *= 1.0-self.settings.syllable_decay_rate;
                    } else {
                        break
                    }
                }
                None => { break }
            }
        }

        word.syllables = match syllable_count {
            1 => {
                get_random_from_weighted(&self.first_syllable_list)
            },
            2 => {
                let mut temp_word = get_random_from_weighted(&self.first_syllable_list);
                temp_word.push_str(&get_random_from_weighted(&self.last_syllable_list));
                temp_word
            },
            _ => {
                let mut temp_word = get_random_from_weighted(&self.first_syllable_list);
                for _ in 1..(syllable_count - 1) {
                    temp_word.push_str(&get_random_from_weighted(&self.normal_syllable_list));
                }
                temp_word.push_str(&get_random_from_weighted(&self.last_syllable_list));
                temp_word
            },
        };
    }

    fn generate_graphemes(&self, word: &mut Word) {

    }
    fn rewrite_syllables(&self, word: &mut Word) {

    }
    fn rewrite_graphemes(&self, word: &mut Word) {

    }
    fn mark_syllable_rejects(&self, word: &mut Word) {

    }
    fn mark_grapheme_rejects(&self, word: &mut Word) {

    }
}

fn get_random_from_weighted(values : &Vec<Weighted<String>>) -> String {

    let mut local_values : Vec<Weighted<String>> = values.clone();

    let selector : WeightedChoice<String> = WeightedChoice::new(&mut local_values);
    let mut rng = rand::thread_rng();

    selector.ind_sample(&mut rng)
}
