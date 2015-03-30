#![feature(collections)]

extern crate rustc_serialize;

use std::error::Error;
use rustc_serialize::*;
use std::io::prelude::*;
use std::fs::File;

pub fn load_config(config_name :&str) -> String {
    let mut config_file = File::open(config_name).unwrap();

    let mut file_buffer : Vec<u8> = Vec::new();

    match config_file.read_to_end(&mut file_buffer) {
        Ok(_) => (),
        Err(error) => panic!("Error reading config: {}", Error::description(&error)),
    };

    match String::from_utf8(file_buffer) {
        Ok(result) => result,
        Err(error) => panic!("Error converting config u8 buffer to a String: {}", Error::description(&error)),
    }
}
