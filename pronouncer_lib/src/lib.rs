#![warn(clippy::all)]

extern crate bincode;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io::BufWriter;
use std::io::Cursor;

use bincode::deserialize;
use hashbrown::HashMap;
use hound::{SampleFormat, WavSpec};

mod phoneme;
use phoneme::Phoneme;

lazy_static! {
    static ref DICT: HashMap<String, Vec<Phoneme>> = deserialize(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/dict_serialized.bin"
    )))
    .unwrap();
    static ref WAV_FILES: HashMap<Phoneme, Vec<i16>> =
        deserialize(include_bytes!(concat!(env!("OUT_DIR"), "/wavs.bin"))).unwrap();
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

pub fn words_to_wav(words: Vec<&str>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut output = Vec::new();
    let writer = BufWriter::new(Cursor::new(&mut output));
    let mut output_wav = hound::WavWriter::new(
        writer,
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
                'a'...'z' | '\'' => true,
                _ => false,
            })
            .collect();
        let symbols: Vec<Phoneme> = match DICT.get(&word) {
            Some(x) => x.to_vec(),
            None => word
                .chars()
                .flat_map(|c| DICT.get(&c.to_string()).unwrap())
                .cloned()
                .collect(),
        };

        for symbol in symbols.iter() {
            for sample in WAV_FILES.get(symbol).unwrap() {
                output_wav.write_sample(*sample)?;
            }
        }
    }
    output_wav.finalize()?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::words_to_wav;
    #[test]
    fn make_wav() {
        words_to_wav("This is a test".split_whitespace().collect()).unwrap();
    }
}
