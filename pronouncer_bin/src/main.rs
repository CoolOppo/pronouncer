#![warn(clippy::all)]

extern crate bincode;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use pronouncer_lib;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut words: Vec<String> = args[1..].to_vec();
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

    if cfg!(target_os = "windows") {
        let out = Command::new("ffplay")
            .arg("output.wav")
            .output()
            .unwrap_or_else(|_| panic!("Failed to open output.wav"));
        if !out.stderr.is_empty() {
            println!("{}", std::str::from_utf8(&out.stderr).unwrap());
        }
    } else {
        let out = Command::new("sh")
            .arg("-c")
            .arg("xdg-open output.wav")
            .output()
            .unwrap_or_else(|_| panic!("Failed to open output.wav"));
        if !out.stderr.is_empty() {
            println!("{}", std::str::from_utf8(&out.stderr)?);
        }
    }

    Ok(())
}
