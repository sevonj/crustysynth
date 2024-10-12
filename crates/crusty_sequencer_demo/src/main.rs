mod midisource;

use std::{fs::File, path::PathBuf, sync::Arc, thread, time::Duration};

use clap::Parser;
use crustysynth::midifile::MidiFile;
use midisource::MidiSource;
use rodio::{OutputStream, Sink};
use rustysynth::SoundFont;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    midi: PathBuf,
    #[arg(short, long)]
    font: PathBuf,
}

fn main() {
    let args = Args::parse();

    let font = match open_font_file(args.font.clone()) {
        Ok(soundfont) => Arc::new(soundfont),
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let midi = match open_midi_file(args.midi.clone()) {
        Ok(midifile) => midifile,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let (_stream, stream_handle) = OutputStream::try_default().expect("Could not create stream");
    let sink = Sink::try_new(&stream_handle).expect("Could not create sink");
    let midisource = MidiSource::new(&font, midi);
    sink.append(midisource);
    sink.play();
    while !sink.empty() {
        thread::sleep(Duration::from_millis(100));
    }
}

fn open_font_file(path: PathBuf) -> anyhow::Result<SoundFont> {
    let mut file = File::open(path)?;
    Ok(SoundFont::new(&mut file)?)
}

fn open_midi_file(path: PathBuf) -> anyhow::Result<MidiFile> {
    Ok(MidiFile::try_from(File::open(path)?)?)
}
