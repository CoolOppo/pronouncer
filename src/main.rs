extern crate bincode;
extern crate serde_derive;

use bincode::deserialize;
use hashbrown::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let words = &args[1..];

    let dict: HashMap<String, Vec<String>> = deserialize(include_bytes!("../ser.bin"))?;
    let mut concat_file = String::new();

    for word in words.iter() {
        let symbols = dict
            .get(word)
            .unwrap_or_else(|| panic!("Could not find {} in dictionary.", word));

        for symbol in symbols.iter() {
            match symbol.as_str() {
                "AA" => concat_file.push_str("file 'audio/AA.wav'\n"),
                "AE" => concat_file.push_str("file 'audio/AE.wav'\n"),
                "AH" => concat_file.push_str("file 'audio/AH.wav'\n"),
                "AO" => concat_file.push_str("file 'audio/AO.wav'\n"),
                "AW" => concat_file.push_str("file 'audio/AW.wav'\n"),
                "AY" => concat_file.push_str("file 'audio/AY.wav'\n"),
                "B" => concat_file.push_str("file 'audio/B.wav'\n"),
                "CH" => concat_file.push_str("file 'audio/CH.wav'\n"),
                "D" => concat_file.push_str("file 'audio/D.wav'\n"),
                "DH" => concat_file.push_str("file 'audio/DH.wav'\n"),
                "EH" => concat_file.push_str("file 'audio/EH.wav'\n"),
                "ER" => concat_file.push_str("file 'audio/ER.wav'\n"),
                "EY" => concat_file.push_str("file 'audio/EY.wav'\n"),
                "F" => concat_file.push_str("file 'audio/F.wav'\n"),
                "G" => concat_file.push_str("file 'audio/G.wav'\n"),
                "HH" => concat_file.push_str("file 'audio/HH.wav'\n"),
                "IH" => concat_file.push_str("file 'audio/IH.wav'\n"),
                "IY" => concat_file.push_str("file 'audio/IY.wav'\n"),
                "JH" => concat_file.push_str("file 'audio/JH.wav'\n"),
                "K" => concat_file.push_str("file 'audio/K.wav'\n"),
                "L" => concat_file.push_str("file 'audio/L.wav'\n"),
                "M" => concat_file.push_str("file 'audio/M.wav'\n"),
                "N" => concat_file.push_str("file 'audio/N.wav'\n"),
                "NG" => concat_file.push_str("file 'audio/NG.wav'\n"),
                "OW" => concat_file.push_str("file 'audio/OW.wav'\n"),
                "OY" => concat_file.push_str("file 'audio/OY.wav'\n"),
                "P" => concat_file.push_str("file 'audio/P.wav'\n"),
                "R" => concat_file.push_str("file 'audio/R.wav'\n"),
                "S" => concat_file.push_str("file 'audio/S.wav'\n"),
                "SH" => concat_file.push_str("file 'audio/SH.wav'\n"),
                "T" => concat_file.push_str("file 'audio/T.wav'\n"),
                "TH" => concat_file.push_str("file 'audio/TH.wav'\n"),
                "UH" => concat_file.push_str("file 'audio/UH.wav'\n"),
                "UW" => concat_file.push_str("file 'audio/UW.wav'\n"),
                "V" => concat_file.push_str("file 'audio/V.wav'\n"),
                "W" => concat_file.push_str("file 'audio/W.wav'\n"),
                "Y" => concat_file.push_str("file 'audio/Y.wav'\n"),
                "Z" => concat_file.push_str("file 'audio/Z.wav'\n"),
                "ZH" => concat_file.push_str("file 'audio/ZH.wav'\n"),
                &_ => (),
            }
        }
    }
    let concat_file_path = std::env::temp_dir().join(Path::new("ffmpeg-concat-list.txt"));
    {
        let mut file = File::create(&concat_file_path).unwrap();
        file.write_all(format!("{}", concat_file).as_bytes())
            .unwrap();
    }
    Command::new("cmd")
        .args(&[
            "/C",
            &format!(
                "ffmpeg -f concat -i {} -c copy -y output.wav",
                concat_file_path.to_string_lossy()
            ),
        ])
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute ffmpeg"));
    std::fs::remove_file(concat_file_path)?;
    Command::new("cmd")
        .args(&["/C", "ffplay output.wav"])
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute ffplay"));

    Ok(())
}
