extern crate rustc_serialize;

use std::error::Error;
use rustc_serialize::*;
use std::io::prelude::*;
use std::fs::File;


#[derive(RustcEncodable, RustcDecodable)]
pub struct OutputSettings {
    show_syllable_generation : bool,
    show_word_rewrites : bool,
    show_full_word_transforms : bool,
    only_mark_rejects : bool,
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Grapheme {
    string : String,
    weight : usize
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct GraphemeGroup {
    name : String,
    graphemes : Vec<Grapheme>,
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Syllable {
    string : String,
    weight : usize,
    only_first_syllable : bool,
    only_last_syllable : bool
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Rewrite {
    pattern : String,
    replace : String
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct RewriteGroup {
    syllable_rewrites : Vec<Rewrite>,
    grapheme_rewrites : Vec<Rewrite>
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct RejectGroup {
    syllable_rejects : Vec<String>,
    grapheme_rejects : Vec<String>
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct GenerateSettings {
    output_file : String,
    syllable_decay_rate : f32,
    max_syllables : usize,
    word_count : usize,
    rewrites_before_rejects : bool,
    debug_use_static_wordlist : bool,
    output_settings : OutputSettings,
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct WordGeneratorConfig {
    settings : GenerateSettings,
    graphemes : Vec<GraphemeGroup>,
    syllables : Vec<Syllable>,
    rewrites : RewriteGroup,
    rejects : RejectGroup
}

pub fn generate_test_config() -> WordGeneratorConfig {
    WordGeneratorConfig {
        settings: GenerateSettings {
            output_file: "test".to_string(),
            syllable_decay_rate: 0.2,
            max_syllables: 10,
            word_count: 10,
            rewrites_before_rejects: false,
            debug_use_static_wordlist: false,
            output_settings: OutputSettings {
                show_syllable_generation: false,
                show_word_rewrites: false,
                show_full_word_transforms: false,
                only_mark_rejects: false,
            },
        },
        graphemes: vec![
            GraphemeGroup {
                name:"c".to_string(),
                graphemes:vec![
                    Grapheme {
                        string: "e".to_string(),
                        weight: 10,
                    }
                ],
            },
        ],
        syllables: vec![
            Syllable{
                string: "CV".to_string(),
                weight: 20,
                only_first_syllable: false,
                only_last_syllable: false,
            }
        ],
        rewrites: RewriteGroup {
            syllable_rewrites: vec![
                Rewrite{
                    pattern:"abc".to_string(),
                    replace:"def".to_string()
                }
            ],
            grapheme_rewrites: vec![
                Rewrite{
                    pattern:"abc".to_string(),
                    replace:"def".to_string()
                }
            ],
        },
        rejects: RejectGroup {
            syllable_rejects: vec![
                "abc".to_string(),
            ],
            grapheme_rejects: vec![
                "abc".to_string(),
            ],

        }
    }
}

pub fn load_config(config_name :&str) -> WordGeneratorConfig {
    let mut config_file = File::open(config_name).unwrap();

    let mut file_buffer : Vec<u8> = Vec::new();

    match config_file.read_to_end(&mut file_buffer) {
        Ok(_) => (),
        Err(error) => panic!("Error reading config: {}", Error::description(&error)),
    };

    let config_encoded: String = match String::from_utf8(file_buffer) {
        Ok(result) => result,
        Err(error) => panic!("Error converting config u8 buffer to a String: {}", Error::description(&error)),
    };

    match json::decode(&config_encoded) {
        Ok(result) => result,
        Err(error) => panic!("Error converting encoded config JSON to struct: {}", error),
    }
}
