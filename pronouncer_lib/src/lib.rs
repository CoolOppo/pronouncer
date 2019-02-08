#![warn(clippy::all)]

extern crate bincode;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::{
    error::Error,
    io::{BufWriter, Cursor},
};

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

        let mut output_samples: Vec<i16> = Vec::new();

        let mut symb_iter = symbols.iter();
        while let Some(first_clip) = symb_iter.next() {
            if let Some(second_clip) = symb_iter.next() {
                let crossfaded = crossfade(
                    WAV_FILES.get(first_clip).unwrap(),
                    WAV_FILES.get(second_clip).unwrap(),
                );

                for sample in crossfaded {
                    output_samples.push(sample);
                }
            } else {
                for sample in WAV_FILES.get(first_clip).unwrap() {
                    output_samples.push(*sample);
                }
            }
        }

        for sample in output_samples.iter() {
            output_wav.write_sample(*sample)?;
        }
    }
    output_wav.finalize()?;

    Ok(output)
}

fn crossfade(first_clip: &[i16], second_clip: &[i16]) -> Vec<i16> {
    let fade_len = (first_clip.len() - 1).min(second_clip.len() - 1) / 2;
    let mut output_clip = Vec::new();
    let first_clip_end = first_clip.len() - 1;
    let fade_start = first_clip_end - (fade_len - 1);
    let second_clip_start_after_fade = fade_len - 1;

    // Write all of the first clip up to the point where we start the crossfade into
    // the output:
    for sample in first_clip.iter().take(fade_start) {
        output_clip.push(*sample);
    }

    // Fade out first clip while fading in second
    for (i, first_clip_sample) in first_clip.iter().enumerate().skip(fade_start) {
        let sample_of_crossfade = i - fade_start;
        let fade_mult = sample_of_crossfade as f64 / fade_len as f64;
        let out_sample: i16 = (((1.0 - fade_mult) * f64::from(*first_clip_sample))
            + (fade_mult * f64::from(second_clip[sample_of_crossfade])))
        .min(f64::from(std::i16::MAX))
        .max(f64::from(std::i16::MIN)) as i16;
        output_clip.push(out_sample);
    }

    // Write rest of second clip
    for sample in second_clip.iter().skip(second_clip_start_after_fade) {
        output_clip.push(*sample);
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
