# Pronouncer

A Rust-based text-to-speech synthesizer that uses the CMU phonetic dictionary and pre-recorded phonemes to generate natural-sounding speech.

## Features

- Text-to-speech synthesis using CMU phonetic dictionary
- High-quality pre-recorded phonemes for natural sound
- Smooth audio transitions using advanced crossfading
- Support for both word and character-by-character pronunciation
- Outputs standard WAV audio files (44.1kHz, 16-bit)
- Static compilation of audio data for standalone binaries

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

The project is organized as a Rust workspace containing two main crates:

### pronouncer_lib
Core library containing the text-to-speech engine:
- `src/lib.rs` - Main library interface and audio processing
- `src/phoneme.rs` - Phoneme enum and conversion functions
- `build.rs` - Build script for processing dictionary and audio files
- `audio/` - Pre-recorded WAV files for each phoneme
- `build/` - Build-time resources including CMU dictionary

### pronouncer_bin
Command-line interface executable:
- `src/main.rs` - CLI implementation
- Handles argument parsing and file I/O

### Key Components

1. **Build System**
   - Processes CMU dictionary at compile time
   - Serializes phoneme WAV files into binary data
   - Generates optimized lookup tables

2. **Phoneme System**
   - 39 distinct phonemes based on CMU dictionary
   - Each phoneme has a corresponding WAV recording
   - Efficient enum-based representation

3. **Audio Processing**
   - 44.1kHz 16-bit mono WAV output
   - Crossfading algorithm for smooth transitions
   - Fileless audio storage - phoneme WAV data is serialized and embedded directly into the binary

4. **Dictionary System**
   - CMU dictionary-based word to phoneme conversion
   - Fallback to character-by-character pronunciation
   - Efficient hashmap-based lookups

## Technical Details

### Build Process
1. The build script (`build.rs`) processes the CMU dictionary and WAV files
2. Dictionary is converted to a binary lookup table using bincode serialization
3. WAV files are serialized and embedded directly into the binary
4. Static initialization provides immediate access to audio data at runtime

### Audio Synthesis Process
1. Input text is normalized and split into words
2. Words are looked up in the CMU dictionary
3. Unknown words fall back to character-by-character pronunciation
4. Phoneme sequences are converted to audio samples
5. Advanced crossfading is applied between phonemes
6. Final audio is written to WAV file

### Performance Considerations
- Audio data is compiled directly into the binary, eliminating runtime file I/O
- Efficient bincode serialization for compact data storage
- High-performance hashmap-based dictionary lookups
- Optimized crossfading algorithm for smooth transitions

## Dependencies

Core dependencies:
- `bincode`: Fast serialization
- `hashbrown`: High-performance hashmaps
- `hound`: WAV file handling
- `lazy_static`: Efficient static initialization
- `serde`: Serialization framework

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Before contributing:
1. Ensure all tests pass: `cargo test`
2. Format code: `cargo fmt`
3. Run clippy: `cargo clippy`

## License

[Add your chosen license here]
