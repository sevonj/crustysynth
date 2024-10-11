use std::{fs::File, path::PathBuf};

use clap::Parser;
use crustysynth::midifile::MidiFile;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let midifile = match parse_midi_file(args.file.clone()) {
        Ok(chunk) => chunk,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    println!("File:     {:?}", args.file);
    println!("Format:   {:?}", midifile.get_format());
    println!("Division: {:?}", midifile.get_division());
    println!("Tracks:   {}", midifile.get_tracks().len());
    for (i, track) in midifile.get_tracks().iter().enumerate() {
        println!("    Track {i}");
        for event in track.get_events() {
            println!("        {:?}", event);
        }
    }
}

fn parse_midi_file(path: PathBuf) -> anyhow::Result<MidiFile> {
    let file = File::open(path)?;

    let midifile = MidiFile::try_from(file)?;

    Ok(midifile)
}
