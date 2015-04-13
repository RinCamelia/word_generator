#![feature(collections)]
#![feature(unicode)]

//modules

mod config;
mod word;

//extern crates

extern crate rand;
extern crate rustc_serialize;
extern crate regex;

//use

use config::*;
use word::*;
use std::io::prelude::*;
//use rustc_serialize::*;    //commented out as it is only used to emit test configs
use std::fs::File;
use rand::distributions::Weighted;

//static variables

static CONFIG_FILE_NAME: &'static str = "main_config.json";

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

    for syllable in &config.syllables {
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

    let grapheme_groups : Vec<(String, Vec<Weighted<String>>)> = transform_graphemes(&config.graphemes);


    let word_factory : WordFactory = WordFactory {
        first_syllable_list : first_syllable_list,
        normal_syllable_list : normal_syllable_list,
        last_syllable_list : last_syllable_list,
        graphemes : grapheme_groups,
        generate_settings : config.generate_settings.clone(),
        rewrites : config.rewrites.clone(),
        rejects : config.rejects.clone(),
    };

    let mut file = File::create(&config.output_settings.output_file).unwrap();


    for word in generate_word_list(&config, &word_factory) {
        match file.write(get_word_graphemes(&word).as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
                Ok(_) => (),
        };
        match file.write("\n".as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
                Ok(_) => (),
        };
    }
}

fn generate_word_list(config : &WordGeneratorConfig, word_factory : &WordFactory) -> Vec<Word> {
    let mut word_list : Vec<Word> = Vec::new();

    for _ in 0..config.output_settings.word_count {

        let mut word : Word = Word {
            syllables : String::new(),
            graphemes : String::new(),
            syllable_rewrite_history : Vec::new(),
            grapheme_rewrite_history : Vec::new(),
            syllable_rejects : Vec::new(),
            grapheme_rejects : Vec::new()
        };

        word_factory.generate_syllables(&mut word);

        if config.generate_settings.rewrites_before_rejects {
            word_factory.rewrite_syllables(&mut word);
            word_factory.mark_syllable_rejects(&mut word);
        } else {
            word_factory.mark_syllable_rejects(&mut word);
            word_factory.rewrite_syllables(&mut word);
        };

        word_factory.generate_graphemes(&mut word);

        if config.generate_settings.rewrites_before_rejects {
            word_factory.rewrite_graphemes(&mut word);
            word_factory.mark_grapheme_rejects(&mut word);
        } else {
            word_factory.mark_grapheme_rejects(&mut word);
            word_factory.rewrite_graphemes(&mut word);
        };
        word_list.push(word);
    }
    word_list
}

//unwraps a list of GraphemeGroups into a tuple list of String and Vec<Weighted<String>>, mostly due to requirement of Weighted for random weighted samples later
fn transform_graphemes(graphemes : &Vec<GraphemeGroup>) -> Vec<(String, Vec<Weighted<String>>)> {

    let mut grapheme_groups : Vec<(String, Vec<Weighted<String>>)> = Vec::new();

    for group in graphemes {
        let mut graphemes_converted : Vec<Weighted<String>> = Vec::new();

        graphemes_converted.extend(
            group.graphemes.iter().map(
                |g: &Grapheme| Weighted{weight: g.weight as u32, item: g.string.clone() }
                )
            );
        grapheme_groups.push((group.name.clone(), graphemes_converted));
    }
    grapheme_groups
}
