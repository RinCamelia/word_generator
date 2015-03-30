#![feature(collections)]

//modules

mod config;
mod word;

//extern crates

extern crate rand;
extern crate rustc_serialize;

//use

use rustc_serialize::*;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

//static variables

static CONFIG_FILE_NAME: &'static str = "json_sample_config.json";

//functions

fn main() {
    let mut file = File::create("words.txt").unwrap();
    let mut config = config::load_config(CONFIG_FILE_NAME);

    for _ in 0..10 {
        let mut word : String = make_word();
        word.push('\n');
        match file.write(word.as_bytes()) {
                Err(error) => panic!("error {} writing to file", Error::description(&error)),
                Ok(_) => (),
        };

    }
}

fn make_word() -> String {
    let vowels : Vec<Weighted<&str>> = vec![
                                        Weighted {weight: 3, item:"i"},
                                        Weighted {weight: 3, item:"e"},
                                        Weighted {weight: 3, item:"é"}]; //, 'a']; //removed leading 'a's as dictated

    let leading_vowel_chance : f32 = 0.25;
    let syllable_decay_mult : f32 = 0.5;

    let mut current_chance_for_syllable : f32 = 0.75;

    let mut rng = rand::thread_rng();
    let mut word : String = String::new();

    //decide if we'll have a leading vowel
    if rng.gen::<f32>() < leading_vowel_chance {
        word.push_str(&get_random_phoneme(&vowels));
    }
    word.push_str(&get_random_syllable()); //minimum 1 normal syllable

    let mut syllable_loop_iter = rng.gen_iter::<f32>();

    loop {
        let result = syllable_loop_iter.next();
        match result {
            Some(x) => {
                if x < current_chance_for_syllable {
                    word.push_str(&get_random_syllable());
                    current_chance_for_syllable *= syllable_decay_mult;
                } else {
                    break
                }
            }
            None => { break }
        }
    }

    word
}

fn get_random_syllable() -> String {
    let consonants : Vec<Weighted<&str>> = vec![
                                        Weighted {weight: 6, item:"p"},
                                        Weighted {weight: 6, item:"m"},
                                        Weighted {weight: 6, item:"n"},
                                        Weighted {weight: 1, item:"ń"},
                                        Weighted {weight: 6, item:"s"},
                                        Weighted {weight: 1, item:"ś"},
                                        Weighted {weight: 6, item:"t"},
                                        Weighted {weight: 1, item:"d"},
                                        Weighted {weight: 4, item:"r"},
                                        Weighted {weight: 2, item:"ts"},
                                        Weighted {weight: 1, item:"tś"},
                                        Weighted {weight: 2, item:"ps"},
                                        Weighted {weight: 1, item:"pś"},
                                        Weighted {weight: 2, item:"mn"}];

    let vowels : Vec<Weighted<&str>> = vec![
                                        Weighted {weight: 3, item:"i"},
                                        Weighted {weight: 3, item:"e"},
                                        Weighted {weight: 3, item:"é"},
                                        Weighted {weight: 3, item:"a"},
                                        Weighted {weight: 1, item:"ié"},
                                        Weighted {weight: 1, item:"ea"}];

    let mut syllable : String = String::new();

    syllable.push_str(&get_random_phoneme(&consonants));
    syllable.push_str(&get_random_phoneme(&vowels));

    syllable
}

fn get_random_phoneme(
    values : &Vec<Weighted<&str>>) -> String {

    let mut local_values : Vec<Weighted<&str>> = values.clone();

    let phoneme_selector : WeightedChoice<&str> = WeightedChoice::new(&mut local_values);
    let mut rng = rand::thread_rng();

    String::from_str(phoneme_selector.ind_sample(&mut rng))
}
