extern crate bincode;
use pronouncer_lib;
use std::env;
use std::error::Error;
use std::io;
use std::process::Command;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut words: Vec<String> = args[1..].to_vec();
    if words.iter().count() < 1 {
        let mut read_words = String::new();
        println!("Enter a string: ");
        io::stdin().read_line(&mut read_words)?;
        words = read_words
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();
    }
    pronouncer_lib::run(words)?;
    if cfg!(target_os = "windows") {
        let out = Command::new("cmd")
            .args(&["/C", "start output.wav"])
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
