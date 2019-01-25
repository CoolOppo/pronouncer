use bincode::serialize;
use hashbrown::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
fn main() -> Result<(), Box<dyn Error>> {
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();
    let mut dict_file_contents = String::new();
    let mut dict_reader = BufReader::new(File::open(Path::new("./cmudict.dict"))?);
    dict_reader.read_to_string(&mut dict_file_contents)?;

    for line in dict_file_contents.lines() {
        let line_split: Vec<&str> = line.split(" ").collect();
        let word = line_split[0];
        let phones: Vec<String> = line_split[1..]
            .iter()
            .map(|s| s.chars().filter(|&c| !"0123456789".contains(c)).collect())
            .collect();

        dict.insert(word.to_string(), phones);
    }

    {
        let mut file = File::create(Path::new("ser.bin")).unwrap();
        file.write_all(&serialize(&dict)?)?;
    }
    Ok(())
}
