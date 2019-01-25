use bincode::serialize;
use hashbrown::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=build/cmudict.dict");
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();
    let mut dict_file_contents = String::new();
    let mut dict_reader = BufReader::new(File::open(Path::new("./build/cmudict.dict"))?);
    dict_reader.read_to_string(&mut dict_file_contents)?;

    for line in dict_file_contents.lines() {
        let line_split: Vec<&str> = line.split(" ").collect();
        let word = line_split[0];
        let phones: Vec<String> = line_split[1..]
            .iter()
            .map(|s| {
                s.chars()
                    .filter(|&c| !"0123456789'.-".contains(c))
                    .collect()
            })
            .collect();

        dict.insert(
            word.to_string()
                .chars()
                .filter(|&c| !"'.-".contains(c))
                .collect(),
            phones,
        );
    }

    {
        let mut file = File::create(Path::new("./build/ser.bin")).unwrap();
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
