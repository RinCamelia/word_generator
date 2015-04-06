#![feature(collections)]

//modules

mod config;
mod word;

//extern crates

extern crate rand;
extern crate rustc_serialize;

//use

use config::*;
use word::*;
use std::io::prelude::*;
//use rustc_serialize::*;    //commented out as it is only used to emit test configs
use std::fs::File;
use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

//static variables

static CONFIG_FILE_NAME: &'static str = "json_sample_config.json";

//functions

fn main() {
    let config: WordGeneratorConfig = load_config(&CONFIG_FILE_NAME);

    //comment other code and uncomment these to emit a new test config
    //let mut file = File::create(&CONFIG_FILE_NAME).unwrap();

    /*match file.write(json::encode(&config).unwrap().as_bytes()) {
        Ok(_) => (),
        Err(err) => panic!("Error writing test config: {}", err),
    }*/

    let mut first_syllable_list : Vec<Weighted<String>> = Vec::new();
    let mut last_syllable_list : Vec<Weighted<String>> = Vec::new();
    let mut normal_syllable_list : Vec<Weighted<String>> = Vec::new();


    //generate weighted lists of syllable configurations in each syllable position

    for syllable in config.syllables {
        let syllable_weighted : Weighted<String> = Weighted{
            weight: syllable.weight as u32,
            item: syllable.string.clone()
        };
        //if the syllable is eligible for all positions
        if !(syllable.only_first_syllable || syllable.only_last_syllable) {
            normal_syllable_list.push(syllable_weighted.clone());
            first_syllable_list.push(syllable_weighted.clone());
            last_syllable_list.push(syllable_weighted.clone());
        }
        //these need to be separate ifs, as marking only first + only last syllable means it can be used for either
        if syllable.only_first_syllable {
            first_syllable_list.push(syllable_weighted.clone());
        }
        if syllable.only_last_syllable {
            last_syllable_list.push(syllable_weighted.clone());
        }

    }

    let mut grapheme_groups : Vec<(String, Vec<Weighted<String>>)> = Vec::new();

    //convert list of graphemes in config into lists of Weighted
    for group in config.graphemes {
        let mut graphemes_converted : Vec<Weighted<String>> = Vec::new();

        graphemes_converted.extend(
            group.graphemes.iter().map(
                |g: &Grapheme| Weighted{weight: g.weight as u32, item: g.string.clone() }
                )
            );
        grapheme_groups.push((group.name.clone(), graphemes_converted));
    }



    let word_factory : WordFactory = WordFactory {
        first_syllable_list : first_syllable_list,
        normal_syllable_list : normal_syllable_list,
        last_syllable_list : last_syllable_list,
        graphemes : grapheme_groups,
        settings : config.settings.clone(),
        rewrites : config.rewrites.clone(),
        rejects : config.rejects.clone(),
    };

    let mut file = File::create(&config.output_settings.output_file).unwrap();


    for _ in 0..config.output_settings.word_count {

        //generate a string of syllables
        // - parse config list of syllables into 3 lists, one for first syllable, one for last, and one for rest



        //apply syllable level rewrites and rejects
        //generate graphemes for each syllable
        //apply grapheme level rewrites and rejects
        //write to file
        let mut word : Word = Word {
                syllables : String::new(),
                graphemes : String::new(),
                syllable_rewrite_history : Vec::new(),
                grapheme_rewrite_history : Vec::new(),
                syllable_rejects : Vec::new(),
                grapheme_rejects : Vec::new()
            };
        word_factory.generate_syllables(&mut word);
        match file.write(word.syllables.as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
                Ok(_) => (),
        };
        match file.write("\n".as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
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

fn get_random_phoneme(values : &Vec<Weighted<&str>>) -> String {

    let mut local_values : Vec<Weighted<&str>> = values.clone();

    let phoneme_selector : WeightedChoice<&str> = WeightedChoice::new(&mut local_values);
    let mut rng = rand::thread_rng();

    String::from_str(phoneme_selector.ind_sample(&mut rng))
}
