//modules

mod config;
mod word;

//extern crates

extern crate rand;
extern crate rustc_serialize;
extern crate regex;
extern crate unicode_segmentation;

//use

use unicode_segmentation::UnicodeSegmentation;
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

    write_list(&generate_word_list(&config, &word_factory), &config);
}


fn write_list(word_list: &Vec<Word>, config : &WordGeneratorConfig) {
    let mut file = File::create(&config.output_settings.output_file).unwrap();

    for word in word_list {
        let mut result : String = format!("{}{}", "Word: ", get_word_graphemes(&word));

        if config.output_settings.show_syllable_strings {
            result = format!("{} ({} syllable(s) {})", result, &word.syllable_count, get_word_syllables(&word));
        }

        //note: we are specifically continue-ing out of this loop in a sub-if to implement mark vs drop behavior
        if word.syllable_rejects.len() > 0 || word.grapheme_rejects.len() > 0 {
            if config.output_settings.only_mark_rejects {
                result = format!("{}{}", result, format_word_rejects(&word));
            }
            else {
                continue;
            }
        }

        if config.output_settings.show_word_rewrites {
            let mut show_no_rewrites_message : bool = true;

            if config.output_settings.show_word_rewrites && config.output_settings.show_syllable_strings && word.syllable_rewrite_history.len() > 0 {
                result = format!("{}\n-Syllable rewrites:\n{}", result, format_transforms(&word.syllables, &word.syllable_rewrite_history));
                show_no_rewrites_message = false;
            }

            if config.output_settings.show_word_rewrites && word.grapheme_rewrite_history.len() > 0 {
                result = format!("{}\n-Grapheme rewrites:\n{}", result, format_transforms(&word.graphemes, &word.grapheme_rewrite_history));
                show_no_rewrites_message = false;
            }

            if word.syllable_rewrite_history.len() > 0 || word.grapheme_rewrite_history.len() > 0 {
                result = format!("{}\n", result);
            }
            else if show_no_rewrites_message { //please note, will not put no rewrites applied if syllable display is off and there are applied syllable rewrites - fix later
                result = format!("{} (No rewrites applied)", result);
            }
        }

        result = format!("{}\n", result);

        match file.write(result.as_bytes()) {
                Err(error) => panic!("error {} writing to file", error),
                Ok(_) => (),
        };
    }
}


fn format_transforms(original : &String, rewrite_history : &Vec<(Rewrite, String)>) -> String {
    let mut result : String = "---------\n".to_string();
    let mut current_previous_word : &String = &original.clone();

    for rewrite in rewrite_history {
        result = format!("-{}\n", format_individual_transform(&rewrite, &current_previous_word));
        current_previous_word = &rewrite.1;
    }

    format!("{}-final: {}\n---------", result, current_previous_word)
}


fn format_individual_transform(transform : &(Rewrite, String), previous : &String) -> String {
    format!("{} --> {} ({} --> {})", previous, transform.1, transform.0.pattern, transform.0.replace)
}


fn format_word_rejects(word : &Word) -> String {
    if word.syllable_rejects.len() == 0 && word.grapheme_rejects.len() == 0 { return String::new(); }
    let mut result : String = " (rejected due to ".to_string();

    match word.syllable_rejects.len() {
        0 => (),
        1 => {
            result.push_str("syllable reject ");
            result.push_str(&word.syllable_rejects[0]);
        },
        2 => {
            result.push_str("syllable rejects ");
            result.push_str(&word.syllable_rejects[0]);
            result.push_str("and ");
            result.push_str(&word.syllable_rejects[1]);
        },
        _ => {
            result.push_str("syllable rejects ");

            //right now, rust has no easy way to trim arbitrary numbers of characters off the end of a string - so i have to count manually and stop adding commas at the last reject entry, otherwise there will be an errant ", " added to the end
            let mut count : usize = 0;
            let length = word.syllable_rejects.len();
            for reject in &word.syllable_rejects {
                result.push_str(&reject);
                if count < length - 1 {
                    result.push_str(", ");
                }

                count = count + 1;
            }

        },
    }

    if word.syllable_rejects.len() > 0 && word.grapheme_rejects.len() > 0 {
        result.push_str(", and due to ");
    }

    match word.grapheme_rejects.len() {
        0 => (),
        1 => {
            result.push_str("grapheme reject ");
            result.push_str(&word.grapheme_rejects[0]);

        },
        2 => {
            result.push_str("grapheme rejects ");
            result.push_str(&word.grapheme_rejects[0]);
            result.push_str("and ");
            result.push_str(&word.grapheme_rejects[1]);
        },
        _ => {
            result.push_str("grapheme rejects ");

            //right now, rust has no easy way to trim arbitrary numbers of characters off the end of a string
            //so i have to count manually and stop adding commas at the last reject entry, otherwise there
            //will be an errant ", " added to the end
            let mut count : usize = 0;
            let length = word.grapheme_rejects.len();

            for reject in &word.grapheme_rejects {
                result.push_str(&reject);
                if count < length - 1 {
                    result.push_str(", ");
                }
                count = count + 1;
            }
        },
    }

    result.push_str(")");
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
            grapheme_rejects : Vec::new(),
            syllable_count : 0
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
