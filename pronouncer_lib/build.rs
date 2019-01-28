#![warn(clippy::all)]

#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

use bincode::serialize;
use hashbrown::HashMap;

include!("./src/phoneme.rs");

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=./pronouncer_lib/build.rs");
    println!("cargo:rerun-if-changed=./pronouncer_lib/Cargo.lock");
    println!("cargo:rerun-if-changed=./pronouncer_lib/build/cmudict.dict");
    let linkage = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or(String::new());
    if linkage.contains("crt-static") {
        println!("the C runtime will be statically linked");
    } else {
        println!("the C runtime will be dynamically linked");
    }

    let mut dict: HashMap<String, Vec<Phoneme>> = HashMap::new();
    let mut dict_file_contents = String::new();
    let mut dict_reader = BufReader::new(File::open(Path::new("./build/cmudict.dict"))?);
    dict_reader.read_to_string(&mut dict_file_contents)?;

    for line in dict_file_contents.lines() {
        let line_split: Vec<&str> = line.split(" ").collect();
        let word = line_split[0];
        let phones: Vec<Phoneme> = line_split[1..]
            .iter()
            .map(|s| {
                s.chars()
                    .filter(|&c| match c {
                        'A'...'Z' => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect::<Vec<String>>()
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| get_phoneme(&s))
            .collect();

        dict.insert(
            word.to_string()
                .chars()
                .filter(|&c| match c {
                    'a'...'z' | 'A'...'Z' => true,
                    _ => false,
                })
                .collect(),
            phones,
        );
    }

    {
        let mut file = File::create(Path::new("./build/dict_serialized.bin")).unwrap();
        file.write_all(&serialize(&dict)?)?;
    }

    // Read wav_files into memory and output a binary containing
    // all of them as a single file to compile be included
    let mut wav_files: HashMap<String, Vec<u8>> = HashMap::new();
    for entry in fs::read_dir("./audio")? {
        let entry = entry?;
        wav_files.insert(
            String::from(entry.file_name().to_string_lossy()),
            std::fs::read(&entry.path())?,
        );
    }
    {
        let mut file = File::create(Path::new("./build/wavs.bin")).unwrap();
        file.write_all(&serialize(&wav_files)?)?;
    }

    Ok(())
}
