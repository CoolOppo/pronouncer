extern crate bincode;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
use bincode::deserialize;
use hashbrown::HashMap;
use hound::{SampleFormat, WavReader, WavSpec};
use std::error::Error;
mod phoneme;
use std::io::BufReader;
include!("phoneme.rs");
lazy_static! {
    static ref DICT: HashMap<String, Vec<Phoneme>> =
        deserialize(include_bytes!("../build/ser.bin")).unwrap();
    static ref WAV_FILES: HashMap<String, Vec<i16>> = {
        let files: HashMap<String, Vec<u8>> =
            deserialize(include_bytes!("../build/wavs.bin")).unwrap();
        let mut wavs: HashMap<String, Vec<i16>> = HashMap::new();
        for (file_name, data) in files {
            let reader = BufReader::new(&data[..]);
            let mut wav_reader = WavReader::new(reader).unwrap();
            wavs.insert(
                file_name,
                wav_reader.samples().map(|r| r.unwrap()).collect(),
            );
        }
        wavs
    };
}

// temporarily keeping this here
// use std::env;
// use std::ffi::CStr;
// use std::ffi::CString;
// use std::io;
// use std::process::Command;
// use libc::c_char;
// for converting TO c string:
// CString::new("data data data data").unwrap().as_ptr()
// #[no_mangle]
// pub extern "C" fn testeringus(_string: *const c_char) {}
// converting FROM c string:
// let c_str = unsafe { CStr::from_ptr(string).to_string_lossy().to_owned() };
// println!("{}", c_str);

pub fn run(words: Vec<&str>) -> Result<(), Box<dyn Error>> {
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
            .filter(|&c| match c {
                'a'...'z' | 'A'...'Z' => true,
                _ => false,
            })
            .collect();
        let symbols = DICT
            .get(&word)
            .unwrap_or_else(|| panic!("Could not find \"{}\" in dictionary.", word));

        for symbol in symbols.iter() {
            for sample in match symbol {
                Phoneme::AA => WAV_FILES.get("AA.wav").unwrap(),
                Phoneme::AE => WAV_FILES.get("AE.wav").unwrap(),
                Phoneme::AH => WAV_FILES.get("AH.wav").unwrap(),
                Phoneme::AO => WAV_FILES.get("AO.wav").unwrap(),
                Phoneme::AW => WAV_FILES.get("AW.wav").unwrap(),
                Phoneme::AY => WAV_FILES.get("AY.wav").unwrap(),
                Phoneme::B => WAV_FILES.get("B.wav").unwrap(),
                Phoneme::CH => WAV_FILES.get("CH.wav").unwrap(),
                Phoneme::D => WAV_FILES.get("D.wav").unwrap(),
                Phoneme::DH => WAV_FILES.get("DH.wav").unwrap(),
                Phoneme::EH => WAV_FILES.get("EH.wav").unwrap(),
                Phoneme::ER => WAV_FILES.get("ER.wav").unwrap(),
                Phoneme::EY => WAV_FILES.get("EY.wav").unwrap(),
                Phoneme::F => WAV_FILES.get("F.wav").unwrap(),
                Phoneme::G => WAV_FILES.get("G.wav").unwrap(),
                Phoneme::HH => WAV_FILES.get("HH.wav").unwrap(),
                Phoneme::IH => WAV_FILES.get("IH.wav").unwrap(),
                Phoneme::IY => WAV_FILES.get("IY.wav").unwrap(),
                Phoneme::JH => WAV_FILES.get("JH.wav").unwrap(),
                Phoneme::K => WAV_FILES.get("K.wav").unwrap(),
                Phoneme::L => WAV_FILES.get("L.wav").unwrap(),
                Phoneme::M => WAV_FILES.get("M.wav").unwrap(),
                Phoneme::N => WAV_FILES.get("N.wav").unwrap(),
                Phoneme::NG => WAV_FILES.get("NG.wav").unwrap(),
                Phoneme::OW => WAV_FILES.get("OW.wav").unwrap(),
                Phoneme::OY => WAV_FILES.get("OY.wav").unwrap(),
                Phoneme::P => WAV_FILES.get("P.wav").unwrap(),
                Phoneme::R => WAV_FILES.get("R.wav").unwrap(),
                Phoneme::S => WAV_FILES.get("S.wav").unwrap(),
                Phoneme::SH => WAV_FILES.get("SH.wav").unwrap(),
                Phoneme::T => WAV_FILES.get("T.wav").unwrap(),
                Phoneme::TH => WAV_FILES.get("TH.wav").unwrap(),
                Phoneme::UH => WAV_FILES.get("UH.wav").unwrap(),
                Phoneme::UW => WAV_FILES.get("UW.wav").unwrap(),
                Phoneme::V => WAV_FILES.get("V.wav").unwrap(),
                Phoneme::W => WAV_FILES.get("W.wav").unwrap(),
                Phoneme::Y => WAV_FILES.get("Y.wav").unwrap(),
                Phoneme::Z => WAV_FILES.get("Z.wav").unwrap(),
                Phoneme::ZH => WAV_FILES.get("ZH.wav").unwrap(),
            } {
                output_wav.write_sample(*sample)?;
            }
        }
    }
    output_wav.finalize()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_wav() {
        run(vec!("This is a test")).unwrap();
    }
}
