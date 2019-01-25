extern crate bincode;
extern crate serde_derive;

use bincode::deserialize;
use hashbrown::HashMap;
use hound::{SampleFormat, WavReader, WavSpec};
use std::env;
use std::error::Error;
use std::io;
use std::io::BufReader;
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

    let dict: HashMap<String, Vec<String>> =
        deserialize(include_bytes!("../build/ser.bin")).unwrap();
    let wav_files = {
        let wav_files: HashMap<String, Vec<u8>> =
            deserialize(include_bytes!("../build/wavs.bin")).unwrap();
        let mut wavs: HashMap<String, Vec<i16>> = HashMap::new();
        for (file_name, data) in wav_files {
            let reader = BufReader::new(&data[..]);
            let mut wav_reader = WavReader::new(reader).unwrap();
            wavs.insert(
                file_name,
                wav_reader.samples().map(|r| r.unwrap()).collect(),
            );
        }
        wavs
    };

    let mut output_wav = hound::WavWriter::create(
        "output.wav",
        WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        },
    )?;
    for word in words.iter() {
        let word: String = word
            .to_lowercase()
            .chars()
            .filter(|&c| !"'.-".contains(c))
            .collect();
        let symbols = dict
            .get(&word)
            .unwrap_or_else(|| panic!("Could not find \"{}\" in dictionary.", word));

        for symbol in symbols.iter() {
            for sample in match symbol.as_str() {
                "AA" => wav_files.get("AA.wav").unwrap(),
                "AE" => wav_files.get("AE.wav").unwrap(),
                "AH" => wav_files.get("AH.wav").unwrap(),
                "AO" => wav_files.get("AO.wav").unwrap(),
                "AW" => wav_files.get("AW.wav").unwrap(),
                "AY" => wav_files.get("AY.wav").unwrap(),
                "B" => wav_files.get("B.wav").unwrap(),
                "CH" => wav_files.get("CH.wav").unwrap(),
                "D" => wav_files.get("D.wav").unwrap(),
                "DH" => wav_files.get("DH.wav").unwrap(),
                "EH" => wav_files.get("EH.wav").unwrap(),
                "ER" => wav_files.get("ER.wav").unwrap(),
                "EY" => wav_files.get("EY.wav").unwrap(),
                "F" => wav_files.get("F.wav").unwrap(),
                "G" => wav_files.get("G.wav").unwrap(),
                "HH" => wav_files.get("HH.wav").unwrap(),
                "IH" => wav_files.get("IH.wav").unwrap(),
                "IY" => wav_files.get("IY.wav").unwrap(),
                "JH" => wav_files.get("JH.wav").unwrap(),
                "K" => wav_files.get("K.wav").unwrap(),
                "L" => wav_files.get("L.wav").unwrap(),
                "M" => wav_files.get("M.wav").unwrap(),
                "N" => wav_files.get("N.wav").unwrap(),
                "NG" => wav_files.get("NG.wav").unwrap(),
                "OW" => wav_files.get("OW.wav").unwrap(),
                "OY" => wav_files.get("OY.wav").unwrap(),
                "P" => wav_files.get("P.wav").unwrap(),
                "R" => wav_files.get("R.wav").unwrap(),
                "S" => wav_files.get("S.wav").unwrap(),
                "SH" => wav_files.get("SH.wav").unwrap(),
                "T" => wav_files.get("T.wav").unwrap(),
                "TH" => wav_files.get("TH.wav").unwrap(),
                "UH" => wav_files.get("UH.wav").unwrap(),
                "UW" => wav_files.get("UW.wav").unwrap(),
                "V" => wav_files.get("V.wav").unwrap(),
                "W" => wav_files.get("W.wav").unwrap(),
                "Y" => wav_files.get("Y.wav").unwrap(),
                "Z" => wav_files.get("Z.wav").unwrap(),
                "ZH" => wav_files.get("ZH.wav").unwrap(),
                &_ => panic!(),
            } {
                output_wav.write_sample(*sample)?;
            }
        }
    }
    output_wav.finalize()?;

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
            println!("{}", std::str::from_utf8(&out.stderr).unwrap());
        }
    }

    Ok(())
}
