extern crate rand;

use config::*;
use rand::*;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use regex::Regex;
use regex::NoExpand;


pub struct Word {
    pub syllables : String,
    pub graphemes : String,
    pub syllable_rewrite_history : Vec<(Rewrite, String)>,
    pub grapheme_rewrite_history : Vec<(Rewrite, String)>,
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

    pub generate_settings : GenerateSettings,
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
                    if x < current_chance_for_syllable && syllable_count < self.generate_settings.max_syllables {
                        syllable_count = syllable_count + 1;
                        current_chance_for_syllable *= 1.0-self.generate_settings.syllable_decay_rate;
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
        assert!(!word.syllables.is_empty(), "Cannot call generate_graphemes on a word without a syllable string");

        let mut grapheme_vector = Vec::new();
        for grapheme in get_word_syllables(&word).graphemes(false) {
            grapheme_vector.push(syllable_element_to_random_grapheme(&self.graphemes, &String::from_str(&grapheme)));
        }

        word.graphemes = grapheme_vector.iter().fold(String::new(), |accumulator : String, character| { let mut new_str = String::from_str(&accumulator); new_str.push_str(&character); new_str});
    }
    fn rewrite_syllables(&self, word: &mut Word) {
        for rewrite in &self.rewrites.syllable_rewrites {
            let rewritten_string = apply_single_rewrite(&rewrite.pattern, &rewrite.replace, &get_word_syllables(&word));
            match rewritten_string {
                Some(result) => word.syllable_rewrite_history.push((rewrite.clone(), result)),
                _ => ()
            }
        }
    }
    fn rewrite_graphemes(&self, word: &mut Word) {
        for rewrite in &self.rewrites.grapheme_rewrites {
            let rewritten_string = apply_single_rewrite(&rewrite.pattern, &rewrite.replace, &get_word_graphemes(&word));
            match rewritten_string {
                Some(result) => word.grapheme_rewrite_history.push((rewrite.clone(), result)),
                _ => ()
            }
        }
    }
    fn mark_syllable_rejects(&self, word: &mut Word) {
        for reject in &self.rejects.syllable_rejects {
            let reject_regex : Regex = match Regex::new(&reject) {
                Ok(res) => res,
                Err(err) => panic!("Error '{}' with regex '{}' in a reject, please verify that it is valid", err, &reject)
            };
            if reject_regex.is_match(&get_word_syllables(&word)) {
                word.syllable_rejects.push(reject.clone());
            }
        }
    }
    fn mark_grapheme_rejects(&self, word: &mut Word) {
        for reject in &self.rejects.grapheme_rejects {
            let reject_regex : Regex = match Regex::new(&reject) {
                Ok(res) => res,
                Err(err) => panic!("Error '{}' with regex '{}' in a reject, please verify that it is valid", err, &reject)
            };
            if reject_regex.is_match(&get_word_graphemes(&word)) {
                word.grapheme_rejects.push(reject.clone());
            }
        }
    }
}

fn apply_single_rewrite(rewrite : &String, replace : &String, source : &String) -> Option<String> {
    let rewrite_regex : Regex = match Regex::new(&rewrite) {
        Ok(res) => res,
        Err(err) => panic!("Error '{}' with regex '{}' in a rewrite, please verify that it is valid", err, &rewrite)
    };
    match rewrite_regex.is_match(&source) {
        true => Some(rewrite_regex.replace_all(&source, NoExpand(replace))),
        false => None,
    }
}

fn syllable_element_to_random_grapheme(grapheme_groups : &Vec<(String, Vec<Weighted<String>>)>, syllable_element : &String) -> String {
    let matching_grapheme_group : Vec<Weighted<String>> = match grapheme_groups.iter().filter(|ref i| i.0 == *syllable_element).map(|ref i| i.1.clone()).last() {
        Some(res) => res,

        None => panic!("No grapheme group found for syllable element {}, check your syllable sets and rewrites", &syllable_element)
    };
    get_random_from_weighted(&matching_grapheme_group)
}

fn get_random_from_weighted(values : &Vec<Weighted<String>>) -> String {

    let mut local_values : Vec<Weighted<String>> = values.clone();

    let selector : WeightedChoice<String> = WeightedChoice::new(&mut local_values);
    let mut rng = rand::thread_rng();

    selector.ind_sample(&mut rng)
}

//temporary function to properly obtain rewritten syllables and graphemes for a word, needs to be refactored into a word trait
pub fn get_word_syllables(word : &Word) -> String {
    match word.syllable_rewrite_history.last() {
        Some(result) => result.1.clone(),
        None => word.syllables.clone()
    }
}
pub fn get_word_graphemes(word : &Word) -> String {
    match word.grapheme_rewrite_history.last() {
        Some(result) => result.1.clone(),
        None => word.graphemes.clone()
    }
}
