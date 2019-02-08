#![warn(clippy::all)]

extern crate bincode;

use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Write},
    path::Path,
};

use pronouncer_lib;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut words: Vec<String> = args[1..]
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(String::from)
        .collect();
    if words.iter().count() < 1 {
        let mut read_words = String::new();
        println!("Enter a string: ");
        io::stdin().read_line(&mut read_words)?;
        words = read_words.split_whitespace().map(String::from).collect();
    }

    let wav_bytes = pronouncer_lib::words_to_wav(words.iter().map(|s| s.as_str()).collect())?;
    let output_path = Path::new("output.wav");
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&wav_bytes)?;

    Ok(())
}
