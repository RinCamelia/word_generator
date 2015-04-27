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


    //In-memory, syllables are stored as 'first-only', 'last only', and 'rest of the word' syllable lists.
    //Aside from the impracticality of iterating and manipulating the config data directly every time I
    //want to pick a syllable, I chose this structure as it made implementing syllable construction
    //straightforward while allowing syllables to be constrained to first or last positions. This may end
    //up getting removed later.

    for syllable in &config.syllables {
        let syllable_weighted : Weighted<String> = Weighted{
            weight: syllable.weight as u32,
            item: syllable.string.clone()
        };
        if !(syllable.only_first_syllable || syllable.only_last_syllable) {
            normal_syllable_list.push(syllable_weighted.clone());
            first_syllable_list.push(syllable_weighted.clone());
            last_syllable_list.push(syllable_weighted.clone());
        }
        if syllable.only_first_syllable {
            first_syllable_list.push(syllable_weighted.clone());
        }
        if syllable.only_last_syllable {
            last_syllable_list.push(syllable_weighted.clone());
        }

    }

    //transform_graphemes is used to convert Graphemes to Weighted<String>s. This is done because
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

    write_list_simple(&generate_word_list(&config, &word_factory), &config);
}

fn write_list_simple(word_list: &Vec<Word>, config : &WordGeneratorConfig) {
    let mut file = File::create(&config.output_settings.output_file).unwrap();

    for word in word_list {
        let mut postfix :String = String::new();
        if word.syllable_rejects.len() > 0 || word.grapheme_rejects.len() > 0 {
            if config.output_settings.only_mark_rejects {
                postfix = format_word_rejects(&word);
            } else {
                continue;
            }
        }
        match file.write(get_word_graphemes(&word).as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
                Ok(_) => (),
        };
        match file.write(postfix.as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
                Ok(_) => (),
        };
        match file.write("\n".as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
                Ok(_) => (),
        };
    }
}


//ultimate goal is to emit formatted strings like this:
//qwe: due to syllable rejects []
//qwe: due to grapheme rejects []
//qwe: due to syllable rejects [] and due to grapheme rejects []
fn format_word_rejects(word : &Word) -> String {
    let mut result : String = String::from_str(": due to ");
    if word.syllable_rejects.len() > 0 {
        result.push_str("syllable rejects ");
        result = word.syllable_rejects.iter().fold(result.clone(),
                                                |accumulator : String, character| {
                                                    let mut new_str = String::from_str(&accumulator);
                                                    new_str.push_str(&character);
                                                    new_str.push_str(", ");
                                                    new_str
                                                });
        if word.grapheme_rejects.len() > 0 {
            result.push_str("and grapheme rejects ");
        }
    }
    if word.grapheme_rejects.len() > 0 {
        if word.syllable_rejects.len() == 0 {
            result.push_str("grapheme rejects ");
        }

        result = word.grapheme_rejects.iter().fold(result.clone(),
                                                |accumulator : String, character| {
                                                    let mut new_str = String::from_str(&accumulator);
                                                    new_str.push_str(&character);
                                                    new_str.push_str(", ");
                                                    new_str
                                                });
    }
    //please note: string always ends with ", " because the fold()s above are unaware of what the last reject is
    result
}


fn generate_word_list(config : &WordGeneratorConfig, word_factory : &WordFactory) -> Vec<Word> {
    let mut word_list : Vec<Word> = Vec::new();

    while word_list.iter().filter(|word| word.syllable_rejects.len() == 0 && word.grapheme_rejects.len() == 0).count() < config.output_settings.word_count {

        let mut word : Word = Word {
            syllables : String::new(),
            graphemes : String::new(),
            syllable_rewrite_history : Vec::new(),
            grapheme_rewrite_history : Vec::new(),
            syllable_rejects : Vec::new(),
            grapheme_rejects : Vec::new()
        };

        word_factory.generate_syllables(&mut word);

        //at first glance this doesnt look like it's doing much - marking rejects operate on the final
        //word string so rewriting before or after will make a difference
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

//unwraps a list of GraphemeGroups into a tuple list of String and Vec<Weighted<String>>
//Stored as a separate structure because JSON data would not be stored as desired for config otherwise
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
