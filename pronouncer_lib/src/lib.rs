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

fn crossfade(first_clip: &Vec<i16>, second_clip: &Vec<i16>) -> Vec<i16> {
    let fade_len = (first_clip.len() + second_clip.len()) / 6;
    let mut output_clip = Vec::new();
    let first_clip_end = first_clip.len() - 1;
    let fade_start = first_clip_end - fade_len;
    let second_clip_start_after_fade = fade_len - 1;

    // Write all of the first clip up to the point where we start the crossfade into the output
    for i in 0..fade_start {
        output_clip.push(first_clip[i]);
    }

    // Fade out first clip while fading in second
    for i in fade_start..=first_clip_end {
        let fade_mult = (i - fade_start) as f64 / (first_clip_end - fade_start) as f64;
        let out_sample: i16 = (((1 as f64 - fade_mult) * first_clip[i] as f64)
            + (fade_mult * second_clip[i - fade_start] as f64))
            as i16;
        output_clip.push(out_sample);
    }

    // Write rest of second clip
    for i in second_clip_start_after_fade..second_clip.len() {
        output_clip.push(second_clip[i]);
    }

    output_clip
}

#[cfg(test)]
mod tests {
    use super::words_to_wav;

    #[test]
    fn make_wav() {
        words_to_wav("This is a test".split_whitespace().collect()).unwrap();
    }
}
