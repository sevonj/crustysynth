use std::{
    fs::File,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use clap::Parser;
use crustysynth::{midifile::MidiFile, sequencer::MidiSequencer};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

const SAMPLERATE: u32 = 44100;

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
        Ok(fontfile) => Arc::new(fontfile),
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
    let settings = SynthesizerSettings::new(SAMPLERATE as i32);
    let synthesizer = match Synthesizer::new(&font, &settings) {
        Ok(synth) => synth,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let mut sequencer = MidiSequencer::new(synthesizer);
    sequencer.play_midi_file(midi);

    let sample_time = Duration::from_secs_f64(1.0) / SAMPLERATE;
    let mut prev_sample_instant = Instant::now();
    loop {
        let delta_t = Instant::now() - prev_sample_instant;
        if delta_t >= sample_time {
            if sequencer.render().is_none() {
                break;
            }
            prev_sample_instant += sample_time;
        }
    }
}

fn open_font_file(path: PathBuf) -> anyhow::Result<SoundFont> {
    let mut file = File::open(path)?;
    Ok(SoundFont::new(&mut file)?)
}

fn open_midi_file(path: PathBuf) -> anyhow::Result<MidiFile> {
    Ok(MidiFile::try_from(File::open(path)?)?)
}
