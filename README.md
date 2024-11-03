# Pronouncer

A Rust-based text-to-speech synthesizer that uses the CMU phonetic dictionary and pre-recorded phonemes to generate speech.

## Features

- Text-to-speech synthesis using CMU phonetic dictionary
- High-quality pre-recorded phonemes
- Smooth audio transitions using crossfading
- Support for both word and character-by-character pronunciation
- Outputs standard WAV audio files

## Installation

1. Ensure you have Rust installed (https://rustup.rs/)
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Usage

Run the program with words as arguments:
```bash
cargo run --release -- "hello world"
```

Or run it interactively:
```bash
cargo run --release
Enter a string: hello world
```

The program will generate an `output.wav` file containing the synthesized speech.

## Project Structure

- `pronouncer_lib`: Core library containing the text-to-speech engine
- `pronouncer_bin`: Command-line interface executable
- `audio/`: Pre-recorded phoneme WAV files
- `build/`: Build-time resources including the CMU dictionary

## Technical Details

The system uses the CMU (Carnegie Mellon University) phonetic dictionary system with 39 distinct phonemes. Each phoneme has a corresponding pre-recorded WAV file that is compiled into the binary at build time. The synthesis process involves:

1. Looking up words in the CMU dictionary
2. Converting to phoneme sequences
3. Concatenating phoneme audio with crossfading
4. Outputting the final WAV file

## License

[Add your chosen license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
